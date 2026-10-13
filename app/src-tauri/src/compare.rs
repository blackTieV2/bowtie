//! D-007: validated read-only comparison. This module has no case-data writes.
use crate::case::{read_only, validate, CaseStore};
use rusqlite::{types::ValueRef, Connection};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io::Write, path::Path};
use tempfile::NamedTempFile;

type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Difference {
    pub table: String,
    pub key: String,
    pub field: String,
    pub status: String,
    pub before: Value,
    pub after: Value,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comparison {
    pub left_title: String,
    pub right_title: String,
    pub signature: String,
    pub differences: Vec<Difference>,
}
type Records = BTreeMap<(String, String), BTreeMap<String, Value>>;

// Names and primary-key columns come exclusively from the validated canonical schema.
// All persisted case fields are compared, including positions and supporting references.
fn records(db: &Connection) -> Result<Records> {
    let mut out = Records::new();
    let tables = {
        let mut q = db.prepare("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").map_err(err)?;
        let rows = q.query_map([], |r| r.get::<_, String>(0)).map_err(err)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?
    };
    for table in tables {
        // The schema has already been checked byte-for-byte by case::validate.
        let primary = {
            let mut q = db
                .prepare(&format!("PRAGMA table_info(\"{table}\")"))
                .map_err(err)?;
            let rows = q
                .query_map([], |r| Ok((r.get::<_, String>(1)?, r.get::<_, i64>(5)?)))
                .map_err(err)?;
            let mut keys = rows.collect::<rusqlite::Result<Vec<_>>>().map_err(err)?;
            keys.retain(|(_, position)| *position > 0);
            keys.sort_by_key(|(_, position)| *position);
            keys.into_iter().map(|(name, _)| name).collect::<Vec<_>>()
        };
        if primary.is_empty() {
            return Err("Comparison requires stable record identities".into());
        }
        let mut q = db
            .prepare(&format!("SELECT * FROM \"{table}\""))
            .map_err(err)?;
        let columns = q
            .column_names()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut rows = q.query([]).map_err(err)?;
        while let Some(row) = rows.next().map_err(err)? {
            let mut fields = BTreeMap::new();
            for (i, column) in columns.iter().enumerate() {
                let value = match row.get_ref(i).map_err(err)? {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(n) => json!(n),
                    ValueRef::Real(n) => json!(n),
                    ValueRef::Text(bytes) => {
                        let text = std::str::from_utf8(bytes).map_err(err)?;
                        if column.ends_with("_json") {
                            serde_json::from_str(text).map_err(err)?
                        } else {
                            json!(text)
                        }
                    }
                    ValueRef::Blob(_) => {
                        return Err("Unsupported binary field in case comparison".into())
                    }
                };
                fields.insert(column.clone(), value);
            }
            let key =
                serde_json::to_string(&primary.iter().map(|k| &fields[k]).collect::<Vec<_>>())
                    .map_err(err)?;
            out.insert((table.clone(), key), fields);
        }
    }
    Ok(out)
}
fn differences(left: &Records, right: &Records) -> Vec<Difference> {
    let keys = left
        .keys()
        .chain(right.keys())
        .collect::<std::collections::BTreeSet<_>>();
    let mut result = Vec::new();
    for (table, key) in keys {
        let identity = (table.clone(), key.clone());
        match (left.get(&identity), right.get(&identity)) {
            (Some(before), Some(after)) => {
                for (field, value) in before {
                    if after.get(field) != Some(value) {
                        result.push(Difference {
                            table: table.clone(),
                            key: key.clone(),
                            field: field.clone(),
                            status: "changed".into(),
                            before: value.clone(),
                            after: after.get(field).cloned().unwrap_or(Value::Null),
                        });
                    }
                }
            }
            (before, after) => result.push(Difference {
                table: table.clone(),
                key: key.clone(),
                field: "record".into(),
                status: if before.is_none() { "added" } else { "removed" }.into(),
                before: before.map(|v| json!(v)).unwrap_or(Value::Null),
                after: after.map(|v| json!(v)).unwrap_or(Value::Null),
            }),
        }
    }
    result
}
fn title(db: &Connection) -> Result<String> {
    db.query_row("SELECT title FROM case_info WHERE id=1", [], |r| r.get(0))
        .map_err(err)
}
impl CaseStore {
    pub fn compare_case(&self, second: &Path) -> Result<Comparison> {
        let first = &self
            .active
            .as_ref()
            .ok_or("Open a case before comparing")?
            .db;
        validate(first)?;
        let second = read_only(second)?;
        // Pin all validation/reads to one read transaction. Never retain this connection
        // as the editable session, call backup into it, or expose it to save commands.
        second.execute_batch("BEGIN DEFERRED").map_err(err)?;
        validate(&second)?;
        let left = records(first)?;
        let right = records(&second)?;
        let signature = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(
                    left.iter().collect::<Vec<_>>(),
                    right.iter().collect::<Vec<_>>()
                ))
                .map_err(err)?
            )
        );
        Ok(Comparison {
            left_title: title(first)?,
            right_title: title(&second)?,
            signature,
            differences: differences(&left, &right),
        })
    }
    pub fn write_comparison(
        &self,
        second: &Path,
        destination: &Path,
        expected: &str,
    ) -> Result<()> {
        if !destination.is_absolute()
            || destination.extension().and_then(|v| v.to_str()) != Some("txt")
        {
            return Err("Choose an absolute .txt report destination".into());
        }
        // Protect the second case even when it was opened from a non-.sqlite name,
        // and protect existing aliases/hardlinks by refusing SQLite report targets.
        if destination == second
            || (destination.exists()
                && (destination.canonicalize().map_err(err)?
                    == second.canonicalize().map_err(err)?
                    || std::fs::read(destination)
                        .map_err(err)?
                        .starts_with(b"SQLite format 3\0")))
        {
            return Err("A comparison report cannot overwrite a case file".into());
        }
        let comparison = self.compare_case(second)?;
        if comparison.signature != expected {
            return Err(
                "A case changed since the comparison. Compare again before reporting.".into(),
            );
        }
        let report = serde_json::to_vec_pretty(&comparison).map_err(err)?;
        let mut output =
            NamedTempFile::new_in(destination.parent().ok_or("Choose a report directory")?)
                .map_err(err)?;
        output.write_all(&report).map_err(err)?;
        output.as_file().sync_all().map_err(err)?;
        output.persist(destination).map_err(err)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_only_diff_report_and_stale_rejection() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("second.sqlite");
        let mut store = CaseStore::default();
        let root = store.create("Baseline", "Workshop").unwrap().tree[0]
            .id
            .clone();
        store.save(Some(&file)).unwrap();
        let bytes = std::fs::read(&file).unwrap();
        assert!(store.compare_case(&file).unwrap().differences.is_empty());
        store.add_group(&root, "New group").unwrap();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        let result = store.compare_case(&file).unwrap();
        assert_eq!(result.differences.len(), 1);
        assert_eq!(result.differences[0].status, "removed");
        let report = folder.path().join("diff.txt");
        store
            .write_comparison(&file, &report, &result.signature)
            .unwrap();
        let original_report = std::fs::read(&report).unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(&original_report).unwrap()["differences"][0]["status"],
            "removed"
        );
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            before
        );
        assert_eq!(std::fs::read(&file).unwrap(), bytes);
        store.add_group(&root, "Another group").unwrap();
        assert!(store
            .write_comparison(&file, &report, &result.signature)
            .is_err());
        assert_eq!(std::fs::read(&report).unwrap(), original_report);
        let reader = read_only(&file).unwrap();
        assert!(reader
            .execute("UPDATE case_info SET title='Forbidden'", [])
            .is_err());
    }
    #[test]
    fn invalid_second_case_and_report_alias_preserve_everything() {
        let folder = tempfile::tempdir().unwrap();
        let mut store = CaseStore::default();
        store.create("Active", "Workshop").unwrap();
        let invalid = folder.path().join("invalid.sqlite");
        std::fs::write(&invalid, b"invalid").unwrap();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store.compare_case(&invalid).is_err());
        assert_eq!(std::fs::read(&invalid).unwrap(), b"invalid");
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            before
        );
        let second = folder.path().join("second.txt");
        let normal = folder.path().join("second.sqlite");
        store.save(Some(&normal)).unwrap();
        std::fs::copy(&normal, &second).unwrap();
        let result = store.compare_case(&second).unwrap();
        assert!(store
            .write_comparison(&second, &second, &result.signature)
            .is_err());
        assert_eq!(
            std::fs::read(&normal).unwrap(),
            std::fs::read(&second).unwrap()
        );
    }
    #[test]
    fn compares_changed_fields_and_json_values_deterministically() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("reference.sqlite");
        let mut store = CaseStore::default();
        let root = store.create("Review", "Workshop").unwrap().tree[0]
            .id
            .clone();
        let snapshot = store.add_group(&root, "Original group").unwrap();
        let group = snapshot
            .tree
            .iter()
            .find(|n| n.kind == "group")
            .unwrap()
            .id
            .clone();
        store.save(Some(&file)).unwrap();
        let original_file = std::fs::read(&file).unwrap();
        store
            .edit_entity(
                &group,
                "Changed group",
                "Multiline\nUnicode: 日本語",
                &json!({}),
            )
            .unwrap();
        let first = store.compare_case(&file).unwrap();
        assert_eq!(first.differences.len(), 2);
        assert!(first
            .differences
            .iter()
            .all(|d| d.status == "changed" && d.table == "entity"));
        assert!(first.differences.iter().any(|d| d.field == "title"
            && d.before == "Changed group"
            && d.after == "Original group"));
        assert!(first.differences.iter().any(|d| d.field == "description"
            && d.before == "Multiline\nUnicode: 日本語"
            && d.after == ""));
        assert_eq!(
            first.differences,
            store.compare_case(&file).unwrap().differences
        );
        assert_eq!(
            first.signature,
            store.compare_case(&file).unwrap().signature
        );
        assert_eq!(std::fs::read(&file).unwrap(), original_file);
        // Semantically equal JSON with different whitespace is not a false difference.
        store
            .active
            .as_ref()
            .unwrap()
            .db
            .execute(
                "UPDATE case_info SET metadata_json=? WHERE id=1",
                ["{ \"format_version\": 1, \"application\": \"barrier-atlas\" }"],
            )
            .unwrap();
        assert_eq!(
            first.differences,
            store.compare_case(&file).unwrap().differences
        );
    }
    #[test]
    fn changed_second_case_and_incompatible_schema_are_rejected_without_writes() {
        let folder = tempfile::tempdir().unwrap();
        let second = folder.path().join("reference.sqlite");
        let report = folder.path().join("report.txt");
        let mut store = CaseStore::default();
        store.create("Review", "Workshop").unwrap();
        store.save(Some(&second)).unwrap();
        let original_session = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        let preview = store.compare_case(&second).unwrap();
        let db = Connection::open(&second).unwrap();
        db.execute("UPDATE case_info SET title='New reference'", [])
            .unwrap();
        drop(db);
        let bytes = std::fs::read(&second).unwrap();
        assert!(store
            .write_comparison(&second, &report, &preview.signature)
            .is_err());
        assert!(!report.exists());
        assert_eq!(std::fs::read(&second).unwrap(), bytes);
        let db = Connection::open(&second).unwrap();
        db.execute_batch("DROP TRIGGER matrix_limit").unwrap();
        drop(db);
        let bytes = std::fs::read(&second).unwrap();
        assert!(store.compare_case(&second).is_err());
        assert_eq!(std::fs::read(&second).unwrap(), bytes);
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            original_session
        );
    }
}
