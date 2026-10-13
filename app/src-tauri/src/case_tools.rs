//! Literal text replacement with reviewed values, optimistic concurrency and one transaction.
use crate::case::{copy_database, memory, CaseSnapshot, CaseStore};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub(crate) struct ToolUndo {
    pub db: Connection,
    pub after_signature: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextChange {
    pub entity_id: String,
    pub path: String,
    pub field: String,
    pub before: String,
    pub after: String,
    pub occurrences: usize,
}
#[derive(Serialize)]
pub struct ReplacePreview {
    pub signature: String,
    pub changes: Vec<TextChange>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub snapshot: CaseSnapshot,
    pub changed_fields: usize,
    pub occurrences: usize,
}
fn path(snapshot: &CaseSnapshot, id: &str) -> String {
    let mut names = Vec::new();
    let mut next = Some(id);
    while let Some(id) = next {
        let Some(node) = snapshot.tree.iter().find(|n| n.id == id) else {
            break;
        };
        names.push(node.title.clone());
        next = node.parent_id.as_deref();
    }
    names.reverse();
    names.join(" / ")
}
impl CaseStore {
    pub fn replace_preview(&self, find: &str, replacement: &str) -> Result<ReplacePreview> {
        if find.is_empty() {
            return Err("Enter text to find".into());
        }
        if find.len() > 100_000 || replacement.len() > 100_000 {
            return Err("Search text is too long".into());
        }
        let context = self.export_context()?;
        let mut changes = Vec::new();
        let mut add = |id: &str, entity_path: &str, field: &str, before: &str| {
            let occurrences = before.matches(find).count();
            let after = before.replace(find, replacement);
            let after = if field == "title" || field.ends_with(":title") {
                after.trim().to_string()
            } else {
                after
            };
            if occurrences > 0 && before != after {
                changes.push(TextChange {
                    entity_id: id.into(),
                    path: entity_path.into(),
                    field: field.into(),
                    before: before.into(),
                    after,
                    occurrences,
                });
            }
        };
        add(
            "@case",
            &format!("Case / {}", context.snapshot.title),
            "case:title",
            &context.snapshot.title,
        );
        let catalog = crate::properties::catalog();
        for node in &context.snapshot.tree {
            let entity_path = path(&context.snapshot, &node.id);
            add(&node.id, &entity_path, "title", &node.title);
            add(&node.id, &entity_path, "description", &node.description);
            // Editable textual values are searched, including validated date/enum text.
            // Foreign-key identities, geometry and numeric/boolean values are excluded.
            let fields = catalog["common"]
                .as_array()
                .unwrap()
                .iter()
                .chain(catalog[&node.kind].as_array().unwrap().iter());
            for f in fields.filter(|f| {
                ["text", "textarea", "date", "select"].contains(&f["type"].as_str().unwrap_or(""))
            }) {
                let key = f["key"].as_str().unwrap();
                if let Some(text) = node.properties.get(key).and_then(Value::as_str) {
                    add(&node.id, &entity_path, &format!("property:{key}"), text);
                }
            }
            if let Some(values) = node.properties.get("user_data").and_then(Value::as_object) {
                for (key, value) in values {
                    if context.snapshot.custom_fields[&node.kind][key] == "text" {
                        if let Some(text) = value.as_str() {
                            add(&node.id, &entity_path, &format!("user:{key}"), text);
                        }
                    }
                }
            }
        }
        for lookup in &context.snapshot.lookups {
            let entity_path = format!("Property choices / {} / {}", lookup.table_key, lookup.title);
            add(&lookup.id, &entity_path, "lookup:title", &lookup.title);
            add(&lookup.id, &entity_path, "lookup:code", &lookup.code);
            if let Some(text) = lookup.metadata.get("description").and_then(Value::as_str) {
                add(&lookup.id, &entity_path, "lookup:description", text);
            }
            if let Some(values) = lookup.metadata.get("user_data").and_then(Value::as_object) {
                for (key, value) in values {
                    if context.snapshot.custom_fields[format!("lookup:{}", lookup.table_key)][key]
                        == "text"
                    {
                        if let Some(text) = value.as_str() {
                            add(
                                &lookup.id,
                                &entity_path,
                                &format!("lookup:user:{key}"),
                                text,
                            );
                        }
                    }
                }
            }
        }
        for matrix in &context.snapshot.matrices {
            let entity_path = format!("Risk matrices / {}", matrix.title);
            add(&matrix.id, &entity_path, "matrix:title", &matrix.title);
            for (axis, levels) in [
                ("likelihood", &matrix.likelihood),
                ("severity", &matrix.severity),
            ] {
                for (i, level) in levels.iter().enumerate() {
                    add(
                        &matrix.id,
                        &entity_path,
                        &format!("matrix:{axis}:{i}:code"),
                        &level.code,
                    );
                    add(
                        &matrix.id,
                        &entity_path,
                        &format!("matrix:{axis}:{i}:title"),
                        &level.title,
                    );
                }
            }
            for (i, category) in matrix.categories.iter().enumerate() {
                add(
                    &matrix.id,
                    &entity_path,
                    &format!("matrix:category:{i}:title"),
                    &category.title,
                );
            }
            for (i, cell) in matrix.cells.iter().enumerate() {
                add(
                    &matrix.id,
                    &entity_path,
                    &format!("matrix:cell:{i}:label"),
                    &cell.label,
                );
            }
        }
        changes.sort_by(|a, b| {
            (&a.path, &a.entity_id, &a.field).cmp(&(&b.path, &b.entity_id, &b.field))
        });
        use sha2::{Digest, Sha256};
        let signature = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(context.signature, find, replacement)).map_err(err)?
            )
        );
        Ok(ReplacePreview { signature, changes })
    }
    pub(crate) fn undo_checkpoint(&self) -> Result<Connection> {
        let mut db = memory()?;
        copy_database(&self.active.as_ref().ok_or("No case is open")?.db, &mut db)?;
        Ok(db)
    }
    pub(crate) fn remember_tool_undo(&mut self, db: Connection) -> Result<()> {
        self.tool_undo = Some(ToolUndo {
            db,
            after_signature: self.export_context()?.signature,
        });
        Ok(())
    }
    pub fn replace_apply(
        &mut self,
        find: &str,
        replacement: &str,
        signature: &str,
        selected: &[usize],
    ) -> Result<ToolResult> {
        let preview = self.replace_preview(find, replacement)?;
        if preview.signature != signature {
            return Err("The case changed after preview. Preview again.".into());
        }
        let unique = selected.iter().copied().collect::<BTreeSet<_>>();
        if unique.is_empty()
            || unique.len() != selected.len()
            || unique.iter().any(|i| *i >= preview.changes.len())
        {
            return Err("Select distinct changes from the current preview".into());
        }
        let undo = self.undo_checkpoint()?;
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let mut occurrences = 0;
        let mut lookup_edits = BTreeSet::new();
        let mut matrix_edits = BTreeSet::new();
        let mut lookups = crate::risk::lookups(&undo)?;
        let mut matrices = crate::risk::matrices(&undo)?;
        for i in unique {
            let change = &preview.changes[i];
            match change.field.as_str() {
                "case:title" => {
                    tx.execute("UPDATE case_info SET title=? WHERE id=1", [&change.after])
                        .map_err(err)?;
                }
                field if field.starts_with("lookup:") => {
                    let lookup = lookups
                        .iter_mut()
                        .find(|l| l.id == change.entity_id)
                        .ok_or("Lookup no longer exists")?;
                    match &field[7..] {
                        "title" => lookup.title = change.after.clone(),
                        "code" => lookup.code = change.after.clone(),
                        "description" => lookup.metadata["description"] = json!(change.after),
                        key if key.starts_with("user:") => {
                            lookup.metadata["user_data"][&key[5..]] = json!(change.after)
                        }
                        _ => return Err("Unsupported lookup field".into()),
                    }
                    lookup_edits.insert(change.entity_id.clone());
                }
                field if field.starts_with("matrix:") => {
                    let matrix = matrices
                        .iter_mut()
                        .find(|m| m.id == change.entity_id)
                        .ok_or("Matrix no longer exists")?;
                    let parts = field.split(':').collect::<Vec<_>>();
                    if parts[1] == "title" {
                        matrix.title = change.after.clone();
                    } else {
                        let index = parts[2].parse::<usize>().map_err(err)?;
                        match (parts[1], parts[3]) {
                            ("likelihood", "code") => {
                                matrix.likelihood[index].code = change.after.clone()
                            }
                            ("likelihood", "title") => {
                                matrix.likelihood[index].title = change.after.clone()
                            }
                            ("severity", "code") => {
                                matrix.severity[index].code = change.after.clone()
                            }
                            ("severity", "title") => {
                                matrix.severity[index].title = change.after.clone()
                            }
                            ("category", "title") => {
                                matrix.categories[index].title = change.after.clone()
                            }
                            ("cell", "label") => matrix.cells[index].label = change.after.clone(),
                            _ => return Err("Unsupported matrix field".into()),
                        }
                    }
                    matrix_edits.insert(change.entity_id.clone());
                }
                "title" => {
                    tx.execute(
                        "UPDATE entity SET title=? WHERE id=?",
                        params![change.after, change.entity_id],
                    )
                    .map_err(err)?;
                }
                "description" => {
                    tx.execute(
                        "UPDATE entity SET description=? WHERE id=?",
                        params![change.after, change.entity_id],
                    )
                    .map_err(err)?;
                }
                field if field.starts_with("property:") => {
                    crate::properties::write(
                        &tx,
                        &change.entity_id,
                        &json!({&field[9..]: change.after}),
                    )?;
                }
                field if field.starts_with("user:") => {
                    let value: String = tx.query_row("SELECT value_json FROM entity_property WHERE entity_id=? AND property_key='user_data'", [&change.entity_id], |r| r.get(0)).map_err(err)?;
                    let mut value: Value = serde_json::from_str(&value).map_err(err)?;
                    value[&field[5..]] = json!(change.after);
                    crate::properties::write(&tx, &change.entity_id, &json!({"user_data": value}))?;
                }
                _ => return Err("Unsupported text field".into()),
            }
            occurrences += change.occurrences;
        }
        for lookup in lookups.iter().filter(|l| lookup_edits.contains(&l.id)) {
            crate::risk::write_lookup(&tx, lookup)?;
        }
        for matrix in matrices.iter().filter(|m| matrix_edits.contains(&m.id)) {
            crate::risk::write_matrix(&tx, matrix)?;
        }
        tx.commit().map_err(err)?;
        let snapshot = self.edited()?;
        self.remember_tool_undo(undo)?;
        Ok(ToolResult {
            snapshot,
            changed_fields: selected.len(),
            occurrences,
        })
    }
    pub fn undo_tool(&mut self) -> Result<CaseSnapshot> {
        let checkpoint = self.tool_undo.as_ref().ok_or("No tool operation to undo")?;
        if self.export_context()?.signature != checkpoint.after_signature {
            return Err("Later changes prevent undo. The case is unchanged.".into());
        }
        let mut restored = memory()?;
        copy_database(&checkpoint.db, &mut restored)?;
        crate::case::validate(&restored)?;
        self.active.as_mut().ok_or("No case is open")?.db = restored;
        self.tool_undo = None;
        self.edited()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preview_individual_all_apply_undo_and_stale_detection() {
        let mut s = CaseStore::default();
        let root = s.create("Review", "Workshop").unwrap().tree[0].id.clone();
        let g = s
            .add_group(&root, "Old old Old")
            .unwrap()
            .tree
            .into_iter()
            .find(|n| n.kind == "group")
            .unwrap()
            .id;
        s.edit_entity(
            &g,
            "Old old Old",
            "Old description",
            &json!({"code":"Old-code"}),
        )
        .unwrap();
        let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
        let p = s.replace_preview("Old", "New").unwrap();
        assert_eq!(p.changes.len(), 3);
        assert!(p.changes.iter().all(|c| c.path == "Workshop / Old old Old"));
        assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
        let r = s.replace_apply("Old", "New", &p.signature, &[0]).unwrap();
        assert_eq!(r.changed_fields, 1);
        assert!(s
            .replace_apply("Old", "New", &p.signature, &[1, 2])
            .is_err());
        s.undo_tool().unwrap();
        assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
        let p = s.replace_preview("Old", "New").unwrap();
        let r = s
            .replace_apply("Old", "New", &p.signature, &[0, 1, 2])
            .unwrap();
        assert_eq!(r.changed_fields, 3);
        assert_eq!(r.occurrences, 4);
        assert!(s.replace_preview("Old", "New").unwrap().changes.is_empty());
    }
    #[test]
    fn invalid_title_rolls_back_all_fields_and_preserves_saved_bytes() {
        let mut s = CaseStore::default();
        let root = s.create("Review", "Old").unwrap().tree[0].id.clone();
        s.edit_entity(&root, "Old", "Old", &json!({"code":"Old"}))
            .unwrap();
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("case.sqlite");
        s.save(Some(&file)).unwrap();
        let bytes = std::fs::read(&file).unwrap();
        let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
        let p = s.replace_preview("Old", "").unwrap();
        assert!(s
            .replace_apply("Old", "", &p.signature, &[0, 1, 2])
            .is_err());
        assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
        assert_eq!(std::fs::read(file).unwrap(), bytes);
        assert!(s.replace_apply("Old", "X", &p.signature, &[0, 0]).is_err());
        assert!(s.replace_apply("Old", "X", &p.signature, &[999]).is_err());
    }
    #[test]
    fn title_preview_matches_normalized_apply_and_binds_replacement_text() {
        let mut s = CaseStore::default();
        s.create("Old", "Workshop").unwrap();
        let p = s.replace_preview("Old", " New ").unwrap();
        assert_eq!(p.changes.len(), 1);
        assert_eq!(p.changes[0].after, "New");
        assert!(s
            .replace_apply("Old", "Unreviewed", &p.signature, &[0])
            .is_err());
        let result = s.replace_apply("Old", " New ", &p.signature, &[0]).unwrap();
        assert_eq!(result.snapshot.title, "New");
    }
}
