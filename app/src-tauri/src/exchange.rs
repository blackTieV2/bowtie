//! Original tabular interchange. Preview and apply share typed transactional services.
use crate::case::{validate, CaseSnapshot, CaseStore};
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::Path,
};
use tempfile::NamedTempFile;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub const HEADER: [&str; 8] = [
    "record_type",
    "record_id",
    "parent_id",
    "kind",
    "title",
    "description",
    "sort_order",
    "payload_json",
];
const LIMIT: usize = 20 * 1024 * 1024;
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportIssue {
    pub line: usize,
    pub message: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    pub signature: String,
    pub counts: BTreeMap<String, usize>,
    pub errors: Vec<ImportIssue>,
    pub updates_empty_case_header: bool,
}
#[derive(Serialize)]
pub struct ImportResult {
    pub snapshot: CaseSnapshot,
    pub counts: BTreeMap<String, usize>,
}
#[derive(Serialize)]
pub struct TableExport {
    pub text: String,
    pub signature: String,
}
#[derive(Clone)]
struct Record {
    line: usize,
    cells: Vec<String>,
    payload: Value,
}
impl Record {
    fn typ(&self) -> &str {
        &self.cells[0]
    }
    fn id(&self) -> &str {
        &self.cells[1]
    }
    fn parent(&self) -> &str {
        &self.cells[2]
    }
    fn kind(&self) -> &str {
        &self.cells[3]
    }
}
pub fn encode(rows: &[Vec<String>]) -> String {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|cell| format!("\"{}\"", cell.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\r\n")
        + "\r\n"
}
// Strict CSV state machine: UTF-8/BOM, escaped quotes and embedded newlines.
// Each record retains the physical start line, not an index after filtering.
pub fn decode(text: &str) -> std::result::Result<Vec<(usize, Vec<String>)>, ImportIssue> {
    if text.len() > LIMIT {
        return Err(ImportIssue {
            line: 1,
            message: "Table exceeds 20 MiB".into(),
        });
    }
    let mut chars = text.trim_start_matches('\u{feff}').chars().peekable();
    let (mut rows, mut cells, mut cell) = (Vec::new(), Vec::new(), String::new());
    let (mut quoted, mut closed, mut line, mut start) = (false, false, 1, 1);
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cell.push('"');
                } else {
                    quoted = false;
                    closed = true;
                }
            } else {
                if c == '\n' {
                    line += 1;
                }
                cell.push(c);
            }
            continue;
        }
        match c {
            '"' if cell.is_empty() && !closed => quoted = true,
            ',' => {
                cells.push(std::mem::take(&mut cell));
                closed = false;
            }
            '\r' | '\n' => {
                if c == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                cells.push(std::mem::take(&mut cell));
                rows.push((start, std::mem::take(&mut cells)));
                line += 1;
                start = line;
                closed = false;
            }
            '"' => {
                return Err(ImportIssue {
                    line,
                    message: "Unexpected quote in unquoted field".into(),
                })
            }
            _ if closed => {
                return Err(ImportIssue {
                    line,
                    message: "Characters after a closing quote".into(),
                })
            }
            _ => cell.push(c),
        }
    }
    if quoted {
        return Err(ImportIssue {
            line: start,
            message: "Unclosed quoted field".into(),
        });
    }
    if !cell.is_empty() || !cells.is_empty() || closed {
        cells.push(cell);
        rows.push((start, cells));
    }
    Ok(rows)
}
fn parse(text: &str) -> (Vec<Record>, Vec<ImportIssue>) {
    let rows = match decode(text) {
        Ok(r) => r,
        Err(e) => return (Vec::new(), vec![e]),
    };
    if rows
        .first()
        .map(|(_, r)| r.iter().map(String::as_str).collect::<Vec<_>>())
        != Some(HEADER.to_vec())
    {
        return (
            Vec::new(),
            vec![ImportIssue {
                line: 1,
                message: "Expected the documented eight-column header".into(),
            }],
        );
    }
    let mut records = Vec::new();
    let mut errors = Vec::new();
    let mut identities = BTreeSet::new();
    let catalog = crate::properties::catalog();
    for (line, cells) in rows.into_iter().skip(1) {
        if cells.len() != 8 {
            errors.push(ImportIssue {
                line,
                message: "Expected eight columns".into(),
            });
            continue;
        }
        if cells[1].trim().is_empty() {
            errors.push(ImportIssue {
                line,
                message: "Record identity is required".into(),
            });
        }
        if !["case", "entity", "lookup", "link", "matrix", "assessment"]
            .contains(&cells[0].as_str())
        {
            errors.push(ImportIssue {
                line,
                message: "Unknown record type".into(),
            });
        }
        if cells[0] == "entity" && (catalog.get(&cells[3]).is_none() || cells[3] == "common") {
            errors.push(ImportIssue {
                line,
                message: "Unknown entity kind".into(),
            });
        }
        if !identities.insert((cells[0].clone(), cells[1].clone())) {
            errors.push(ImportIssue {
                line,
                message: "Duplicate record identity".into(),
            });
        }
        if cells[6].parse::<i64>().ok().filter(|n| *n >= 0).is_none() {
            errors.push(ImportIssue {
                line,
                message: "Order must be a non-negative integer".into(),
            });
        }
        let payload = match serde_json::from_str::<Value>(&cells[7]) {
            Ok(v) if v.is_object() => v,
            _ => {
                errors.push(ImportIssue {
                    line,
                    message: "Payload must be a JSON object".into(),
                });
                continue;
            }
        };
        let issue = |message: &str| ImportIssue {
            line,
            message: message.into(),
        };
        if cells[0] == "case" {
            if cells[1] != "case"
                || !cells[2].is_empty()
                || !cells[3].is_empty()
                || !cells[5].is_empty()
                || cells[6] != "0"
                || cells[4].trim().is_empty()
            {
                errors.push(issue("Malformed case header columns"));
            }
            if payload["author"].as_str().is_none()
                || payload["message"].as_str().is_none()
                || !payload["metadata"].is_object()
                || payload["metadata"]["application"] != "barrier-atlas"
                || payload["metadata"]["format_version"] != 1
                || payload
                    .as_object()
                    .unwrap()
                    .keys()
                    .any(|k| !["author", "message", "metadata"].contains(&k.as_str()))
            {
                errors.push(issue("Malformed or unsupported case header payload"));
            }
        }
        if ["case", "lookup", "matrix"].contains(&cells[0].as_str()) && !cells[2].is_empty() {
            errors.push(issue("This record type has no parent column"));
        }
        if ["case", "link", "matrix", "assessment"].contains(&cells[0].as_str())
            && (!cells[5].is_empty() || cells[6] != "0")
        {
            errors.push(issue("Reserved description/order columns must be empty/0"));
        }
        if ["link", "assessment"].contains(&cells[0].as_str()) && !cells[4].is_empty() {
            errors.push(issue("This record type has no title column"));
        }
        if cells[0] == "matrix"
            && (!cells[3].is_empty()
                || payload
                    .as_object()
                    .unwrap()
                    .keys()
                    .any(|k| !["matrix", "metadata"].contains(&k.as_str()))
                || !payload["metadata"].is_object())
        {
            errors.push(issue("Malformed matrix wrapper"));
        }
        if cells[0] == "link"
            && (payload.as_object().unwrap().len() != 1 || payload["targetId"].as_str().is_none())
        {
            errors.push(issue("Link payload must contain only a text targetId"));
        }
        records.push(Record {
            line,
            cells,
            payload,
        });
    }
    if records.iter().filter(|r| r.typ() == "case").count() != 1 {
        errors.push(ImportIssue {
            line: 1,
            message: "Exactly one case header is required".into(),
        });
    }
    if records
        .iter()
        .filter(|r| {
            r.typ() == "entity"
                && r.kind() == "organization"
                && r.id() == "@root"
                && r.parent().is_empty()
        })
        .count()
        != 1
    {
        errors.push(ImportIssue {
            line: 1,
            message: "Exactly one @root organization with no parent is required".into(),
        });
    }
    (records, errors)
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EntityPayload {
    properties: Value,
    effectiveness_id: Option<String>,
    type_key: String,
    metadata: Value,
}
fn definitions_valid(defs: &Value) -> Result<()> {
    let catalog = crate::properties::catalog();
    for (kind, fields) in defs
        .as_object()
        .ok_or("Custom definitions must be an object")?
    {
        let known = catalog.get(kind).is_some() && kind != "common";
        let known_lookup = kind.strip_prefix("lookup:").is_some_and(|key| {
            catalog
                .as_object()
                .unwrap()
                .values()
                .filter_map(Value::as_array)
                .flatten()
                .any(|f| f["table"] == key)
        });
        if !known && !known_lookup {
            return Err("Unknown custom definition kind".into());
        }
        for (key, typ) in fields
            .as_object()
            .ok_or("Custom fields must be an object")?
        {
            if key.trim().is_empty()
                || !["text", "number", "boolean"].contains(&typ.as_str().unwrap_or(""))
            {
                return Err("Invalid custom field name/type".into());
            }
        }
    }
    Ok(())
}
fn apply_records(tx: &Transaction<'_>, records: &[Record]) -> std::result::Result<(), ImportIssue> {
    let fail = |r: &Record, message: String| ImportIssue {
        line: r.line,
        message,
    };
    let root: String = tx
        .query_row("SELECT id FROM entity WHERE kind='organization'", [], |r| {
            r.get(0)
        })
        .map_err(|e| ImportIssue {
            line: 1,
            message: err(e),
        })?;
    let pristine: bool = tx.query_row("SELECT (SELECT count(*) FROM entity)=1 AND NOT EXISTS(SELECT 1 FROM lookup_value) AND NOT EXISTS(SELECT 1 FROM risk_matrix) AND NOT EXISTS(SELECT 1 FROM entity_property)", [], |r| r.get(0)).map_err(|e| ImportIssue { line: 1, message: err(e) })?;
    let root_record = records
        .iter()
        .find(|r| r.typ() == "entity" && r.id() == "@root")
        .ok_or(ImportIssue {
            line: 1,
            message: "Missing organization".into(),
        })?;
    let package_case = records
        .iter()
        .find(|r| r.typ() == "case")
        .ok_or(ImportIssue {
            line: 1,
            message: "Missing case header".into(),
        })?;
    let root_payload: EntityPayload = serde_json::from_value(root_record.payload.clone())
        .map_err(|e| fail(root_record, err(e)))?;
    if !root_payload.properties.is_object() {
        return Err(fail(root_record, "Properties must be an object".into()));
    }
    let defs = root_payload
        .properties
        .get("custom_definitions")
        .cloned()
        .unwrap_or(json!({}));
    definitions_valid(&defs).map_err(|e| fail(root_record, e))?;
    if !pristine {
        let existing: (String, String, String, String) = tx
            .query_row(
                "SELECT title,description,type_key,metadata_json FROM entity WHERE id=?",
                [&root],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .map_err(|e| fail(root_record, err(e)))?;
        let current_defs = crate::properties::definitions(tx).map_err(|e| fail(root_record, e))?;
        if existing.0 != root_record.cells[4]
            || existing.1 != root_record.cells[5]
            || existing.2 != root_payload.type_key
            || serde_json::from_str::<Value>(&existing.3).map_err(|e| fail(root_record, err(e)))?
                != root_payload.metadata
            || current_defs != defs
        {
            return Err(fail(root_record, "A populated case must have matching organization details and custom definitions; import into a new empty case otherwise".into()));
        }
    } else {
        tx.execute("UPDATE entity SET title=?,description=?,type_key=?,metadata_json=?,sort_order=? WHERE id=?", params![root_record.cells[4],root_record.cells[5],root_payload.type_key,root_payload.metadata.to_string(),root_record.cells[6].parse::<i64>().unwrap(),root]).map_err(|e| fail(root_record,err(e)))?;
        if package_case.cells[4].trim().is_empty() {
            return Err(fail(package_case, "Case title is required".into()));
        }
        let application = package_case
            .payload
            .get("metadata")
            .ok_or_else(|| fail(package_case, "Missing case metadata".into()))?;
        if application["application"] != "barrier-atlas" || application["format_version"] != 1 {
            return Err(fail(
                package_case,
                "Unsupported table application/version".into(),
            ));
        }
        tx.execute(
            "UPDATE case_info SET title=?,author=?,message=?,metadata_json=? WHERE id=1",
            params![
                package_case.cells[4],
                package_case.payload["author"]
                    .as_str()
                    .ok_or_else(|| fail(package_case, "Author must be text".into()))?,
                package_case.payload["message"]
                    .as_str()
                    .ok_or_else(|| fail(package_case, "Message must be text".into()))?,
                application.to_string()
            ],
        )
        .map_err(|e| fail(package_case, err(e)))?;
        tx.execute(
            "INSERT INTO entity_property VALUES(?,'custom_definitions',?)",
            params![root, defs.to_string()],
        )
        .map_err(|e| fail(root_record, err(e)))?;
    }
    let map_root = |id: &str| {
        if id == "@root" {
            root.clone()
        } else {
            id.into()
        }
    };
    for r in records.iter().filter(|r| r.typ() == "lookup") {
        let lookup: crate::risk::Lookup =
            serde_json::from_value(r.payload.clone()).map_err(|e| fail(r, err(e)))?;
        if lookup.id != r.id()
            || lookup.table_key != r.kind()
            || lookup.title != r.cells[4]
            || lookup.sort_order.to_string() != r.cells[6]
        {
            return Err(fail(r, "Lookup columns and payload disagree".into()));
        }
        let exists: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM lookup_value WHERE id=?)",
                [r.id()],
                |v| v.get(0),
            )
            .map_err(|e| fail(r, err(e)))?;
        if exists {
            return Err(fail(r, "Duplicate existing lookup identity".into()));
        }
        crate::risk::write_lookup(tx, &lookup).map_err(|e| fail(r, e))?;
        tx.execute(
            "UPDATE lookup_value SET title=? WHERE id=?",
            params![lookup.title, lookup.id],
        )
        .map_err(|e| fail(r, err(e)))?;
    }
    // Resolve hierarchy independent of input ordering; unresolved parents/cycles fail.
    let mut remaining = records
        .iter()
        .filter(|r| r.typ() == "entity" && r.id() != "@root")
        .collect::<Vec<_>>();
    while !remaining.is_empty() {
        let mut progressed = false;
        let mut deferred = Vec::new();
        for r in remaining {
            let parent = map_root(r.parent());
            let exists: bool = tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM entity WHERE id=?)",
                    [&parent],
                    |v| v.get(0),
                )
                .map_err(|e| fail(r, err(e)))?;
            if !exists {
                deferred.push(r);
                continue;
            }
            crate::editing::insert_entity(
                tx,
                r.id(),
                &parent,
                r.kind(),
                &r.cells[4],
                &r.cells[5],
                r.cells[6].parse::<i64>().unwrap(),
            )
            .map_err(|e| fail(r, e))?;
            progressed = true;
        }
        if !progressed {
            return Err(fail(
                deferred[0],
                "Missing parent or hierarchy cycle".into(),
            ));
        }
        remaining = deferred;
    }
    for r in records.iter().filter(|r| r.typ() == "entity") {
        let id = map_root(r.id());
        let payload: EntityPayload =
            serde_json::from_value(r.payload.clone()).map_err(|e| fail(r, err(e)))?;
        if !payload.metadata.is_object() || !payload.properties.is_object() {
            return Err(fail(r, "Metadata and properties must be objects".into()));
        }
        if r.id() == "@root" && !pristine {
            let current: String = tx.query_row("SELECT coalesce(json_group_object(property_key,json(value_json)),'{}') FROM entity_property WHERE entity_id=?", [&id], |v|v.get(0)).map_err(|e|fail(r,err(e)))?;
            if serde_json::from_str::<Value>(&current).map_err(|e| fail(r, err(e)))?
                != payload.properties
            {
                return Err(fail(
                    r,
                    "Organization properties differ; import into a new empty case".into(),
                ));
            }
            continue;
        }
        tx.execute(
            "UPDATE entity SET type_key=?,metadata_json=?,effectiveness_id=? WHERE id=?",
            params![
                payload.type_key,
                payload.metadata.to_string(),
                payload.effectiveness_id,
                id
            ],
        )
        .map_err(|e| fail(r, err(e)))?;
        let mut ordinary = payload.properties.clone();
        if ordinary.get("links").is_some() {
            return Err(fail(
                r,
                "Use dedicated link records instead of an entity links property".into(),
            ));
        }
        ordinary
            .as_object_mut()
            .unwrap()
            .remove("custom_definitions");
        if r.kind() != "organization" && payload.properties.get("custom_definitions").is_some() {
            return Err(fail(r, "Definitions belong to the organization".into()));
        }
        if let Some(position) = ordinary.as_object_mut().unwrap().remove("canvas.position") {
            let x = position["x"].as_f64();
            let y = position["y"].as_f64();
            if ![
                "hazard",
                "top_event",
                "threat",
                "consequence",
                "barrier",
                "escalation_factor",
            ]
            .contains(&r.kind())
                || !x.is_some_and(|v| v.is_finite() && v.abs() <= 1_000_000.0)
                || !y.is_some_and(|v| v.is_finite() && v.abs() <= 1_000_000.0)
            {
                return Err(fail(r, "Invalid saved canvas position".into()));
            }
            tx.execute(
                "INSERT INTO entity_property VALUES(?,'canvas.position',?)",
                params![id, position.to_string()],
            )
            .map_err(|e| fail(r, err(e)))?;
        }
        if let Some(rating) = ordinary.get("effectiveness_id") {
            if rating
                .as_str()
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                != payload.effectiveness_id
            {
                return Err(fail(r, "Effectiveness column/property disagree".into()));
            }
        }
        crate::properties::write(tx, &id, &ordinary).map_err(|e| fail(r, e))?;
        // Preserve absence of an empty definitions property for exact round trips.
        if r.id() == "@root" && payload.properties.get("custom_definitions").is_none() {
            tx.execute("DELETE FROM entity_property WHERE entity_id=? AND property_key='custom_definitions'", [&id]).map_err(|e|fail(r,err(e)))?;
        }
    }
    for r in records.iter().filter(|r| r.typ() == "matrix") {
        let matrix: crate::risk::Matrix =
            serde_json::from_value(r.payload["matrix"].clone()).map_err(|e| fail(r, err(e)))?;
        if matrix.id != r.id() || matrix.title != r.cells[4] {
            return Err(fail(r, "Matrix columns and payload disagree".into()));
        }
        let exists: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM risk_matrix WHERE id=?)",
                [r.id()],
                |v| v.get(0),
            )
            .map_err(|e| fail(r, err(e)))?;
        if exists {
            return Err(fail(r, "Duplicate existing matrix identity".into()));
        }
        crate::risk::write_matrix(tx, &matrix).map_err(|e| fail(r, e))?;
        tx.execute(
            "UPDATE risk_matrix SET title=?,metadata_json=? WHERE id=?",
            params![matrix.title, r.payload["metadata"].to_string(), r.id()],
        )
        .map_err(|e| fail(r, err(e)))?;
    }
    for r in records.iter().filter(|r| r.typ() == "link") {
        let source = map_root(r.parent());
        let target = map_root(
            r.payload["targetId"]
                .as_str()
                .ok_or_else(|| fail(r, "Missing link target".into()))?,
        );
        let key = serde_json::to_string(&(
            r.parent(),
            r.payload["targetId"].as_str().unwrap(),
            r.kind(),
        ))
        .map_err(|e| fail(r, err(e)))?;
        if r.id() != key {
            return Err(fail(
                r,
                "Link identity must match its source, target and relation".into(),
            ));
        }
        tx.execute(
            "INSERT INTO entity_link VALUES(?,?,?)",
            params![source, target, r.kind()],
        )
        .map_err(|e| fail(r, err(e)))?;
    }
    for r in records.iter().filter(|r| r.typ() == "assessment") {
        let a: crate::risk::Assessment =
            serde_json::from_value(r.payload.clone()).map_err(|e| fail(r, err(e)))?;
        let key = serde_json::to_string(&(&a.target_id, &a.matrix_id, &a.phase))
            .map_err(|e| fail(r, err(e)))?;
        if r.id() != key || r.parent() != a.target_id || r.kind() != a.phase {
            return Err(fail(r, "Assessment columns and payload disagree".into()));
        }
        tx.execute("INSERT INTO risk_assessment(target_id,matrix_id,phase,likelihood_position,severity_position) VALUES(?,?,?,?,?)",params![a.target_id,a.matrix_id,a.phase,a.likelihood_position,a.severity_position]).map_err(|e|fail(r,err(e)))?;
    }
    validate(tx).map_err(|e| ImportIssue {
        line: 1,
        message: e,
    })?;
    Ok(())
}
impl CaseStore {
    pub fn table_export(&self) -> Result<TableExport> {
        let context = self.export_context()?;
        let s = &context.snapshot;
        let db = &self.active.as_ref().ok_or("No case is open")?.db;
        let root = &s
            .tree
            .iter()
            .find(|r| r.kind == "organization")
            .ok_or("Missing organization")?
            .id;
        let map = |id: &str| {
            if id == root {
                "@root".to_string()
            } else {
                id.into()
            }
        };
        let mut rows = vec![HEADER.iter().map(|s| s.to_string()).collect::<Vec<_>>()];
        let info: (String, String, String) = db
            .query_row(
                "SELECT author,message,metadata_json FROM case_info WHERE id=1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(err)?;
        rows.push(vec!["case".into(),"case".into(),"".into(),"".into(),s.title.clone(),"".into(),"0".into(),json!({"author":info.0,"message":info.1,"metadata":serde_json::from_str::<Value>(&info.2).map_err(err)?}).to_string()]);
        for n in &s.tree {
            let extra: (String, String) = db
                .query_row(
                    "SELECT type_key,metadata_json FROM entity WHERE id=?",
                    [&n.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(err)?;
            rows.push(vec!["entity".into(),map(&n.id),n.parent_id.as_deref().map(map).unwrap_or_default(),n.kind.clone(),n.title.clone(),n.description.clone(),n.sort_order.to_string(),json!({"properties":n.properties,"effectivenessId":n.effectiveness_id,"typeKey":extra.0,"metadata":serde_json::from_str::<Value>(&extra.1).map_err(err)?}).to_string()]);
        }
        for l in &s.lookups {
            rows.push(vec![
                "lookup".into(),
                l.id.clone(),
                "".into(),
                l.table_key.clone(),
                l.title.clone(),
                "".into(),
                l.sort_order.to_string(),
                serde_json::to_string(l).map_err(err)?,
            ]);
        }
        for l in &s.links {
            let source = map(&l.source_id);
            let target = map(&l.target_id);
            rows.push(vec![
                "link".into(),
                serde_json::to_string(&(&source, &target, &l.relation)).map_err(err)?,
                source,
                l.relation.clone(),
                "".into(),
                "".into(),
                "0".into(),
                json!({"targetId":target}).to_string(),
            ]);
        }
        for m in &s.matrices {
            let (title, metadata): (String, String) = db
                .query_row(
                    "SELECT title,metadata_json FROM risk_matrix WHERE id=?",
                    [&m.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(err)?;
            let mut matrix = m.clone();
            matrix.title = title.clone();
            rows.push(vec!["matrix".into(),m.id.clone(),"".into(),"".into(),title,"".into(),"0".into(),json!({"matrix":matrix,"metadata":serde_json::from_str::<Value>(&metadata).map_err(err)?}).to_string()]);
        }
        for a in &s.assessments {
            rows.push(vec![
                "assessment".into(),
                serde_json::to_string(&(&a.target_id, &a.matrix_id, &a.phase)).map_err(err)?,
                a.target_id.clone(),
                a.phase.clone(),
                "".into(),
                "".into(),
                "0".into(),
                serde_json::to_string(a).map_err(err)?,
            ]);
        }
        Ok(TableExport {
            text: encode(&rows),
            signature: context.signature,
        })
    }
    pub fn read_table(&self, path: &Path) -> Result<Vec<u8>> {
        if !path.is_absolute()
            || !path
                .extension()
                .and_then(|v| v.to_str())
                .is_some_and(|e| ["csv", "xlsx"].contains(&e.to_ascii_lowercase().as_str()))
        {
            return Err("Choose an absolute .csv or .xlsx input".into());
        }
        if std::fs::metadata(path).map_err(err)?.len() > LIMIT as u64 {
            return Err("Input exceeds 20 MiB".into());
        }
        std::fs::read(path).map_err(err)
    }
    pub fn write_table(&self, path: &Path, bytes: &[u8], signature: &str) -> Result<()> {
        if self.export_context()?.signature != signature {
            return Err("The case changed while exporting".into());
        }
        let extension = path.extension().and_then(|v| v.to_str()).unwrap_or("");
        if !path.is_absolute()
            || !["csv", "xlsx"].contains(&extension)
            || bytes.is_empty()
            || bytes.len() > LIMIT
        {
            return Err("Choose an absolute .csv or .xlsx destination (up to 20 MiB)".into());
        }
        if extension == "csv"
            && std::str::from_utf8(bytes).map_err(err)? != self.table_export()?.text
        {
            return Err("CSV payload does not match the live export".into());
        }
        if extension == "xlsx" && !bytes.starts_with(b"PK\x03\x04") {
            return Err("Spreadsheet payload is invalid".into());
        }
        let mut output = NamedTempFile::new_in(path.parent().ok_or("Choose an output directory")?)
            .map_err(err)?;
        output.write_all(bytes).map_err(err)?;
        output.as_file().sync_all().map_err(err)?;
        output.persist(path).map_err(err)?;
        Ok(())
    }
    pub fn import_preview(&self, text: &str) -> Result<ImportPreview> {
        let context = self.export_context()?;
        let signature = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&(context.signature, text)).map_err(err)?)
        );
        let (records, mut errors) = parse(text);
        let parents = records
            .iter()
            .filter(|r| r.typ() == "entity")
            .map(|r| (r.id(), r.parent()))
            .collect::<BTreeMap<_, _>>();
        let existing = context
            .snapshot
            .tree
            .iter()
            .map(|r| r.id.as_str())
            .collect::<BTreeSet<_>>();
        for r in records
            .iter()
            .filter(|r| r.typ() == "entity" && r.id() != "@root")
        {
            if existing.contains(r.id()) {
                errors.push(ImportIssue {
                    line: r.line,
                    message: "Duplicate existing entity identity".into(),
                });
            }
            if !parents.contains_key(r.parent()) && !existing.contains(r.parent()) {
                errors.push(ImportIssue {
                    line: r.line,
                    message: "Missing parent".into(),
                });
            }
            let mut visited = BTreeSet::new();
            let mut current = r.id();
            while let Some(parent) = parents.get(current) {
                if !visited.insert(current) {
                    errors.push(ImportIssue {
                        line: r.line,
                        message: "Hierarchy cycle".into(),
                    });
                    break;
                }
                if parent.is_empty() {
                    break;
                }
                current = parent;
            }
        }
        let mut counts = BTreeMap::new();
        for r in &records {
            if r.typ() != "case" && !(r.typ() == "entity" && r.id() == "@root") {
                *counts
                    .entry(if r.typ() == "entity" && r.kind() == "action" {
                        "action".into()
                    } else {
                        r.typ().into()
                    })
                    .or_insert(0) += 1;
            }
        }
        let mut db = self.undo_checkpoint()?;
        let pristine:bool=db.query_row("SELECT (SELECT count(*) FROM entity)=1 AND NOT EXISTS(SELECT 1 FROM lookup_value) AND NOT EXISTS(SELECT 1 FROM risk_matrix) AND NOT EXISTS(SELECT 1 FROM entity_property)",[],|r|r.get(0)).map_err(err)?;
        if errors.is_empty() {
            let tx = db.transaction().map_err(err)?;
            if let Err(issue) = apply_records(&tx, &records) {
                errors.push(issue);
            }
            // Deliberately drop the transaction: dry-run never alters either case.
        }
        Ok(ImportPreview {
            signature,
            counts,
            errors,
            updates_empty_case_header: pristine,
        })
    }
    pub fn import_apply(&mut self, text: &str, signature: &str) -> Result<ImportResult> {
        let preview = self.import_preview(text)?;
        if preview.signature != signature {
            return Err("The case or table changed after preview. Validate again.".into());
        }
        if !preview.errors.is_empty() {
            return Err(preview
                .errors
                .iter()
                .map(|e| format!("Line {}: {}", e.line, e.message))
                .collect::<Vec<_>>()
                .join("\n"));
        }
        let undo = self.undo_checkpoint()?;
        let (records, _) = parse(text);
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        apply_records(&tx, &records).map_err(|e| format!("Line {}: {}", e.line, e.message))?;
        tx.commit().map_err(err)?;
        let snapshot = self.edited()?;
        self.remember_tool_undo(undo)?;
        Ok(ImportResult {
            snapshot,
            counts: preview.counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn populated() -> CaseStore {
        let mut s = CaseStore::default();
        let root = s.create("Transfer review", "Workshop").unwrap().tree[0]
            .id
            .clone();
        let g = s
            .add_group(&root, "Station, \"A\"")
            .unwrap()
            .tree
            .into_iter()
            .find(|n| n.kind == "group")
            .unwrap()
            .id;
        let snap = s.add_diagram(&g, "Liquid transfer", "Release").unwrap();
        let diagram = snap
            .tree
            .iter()
            .find(|n| n.kind == "diagram")
            .unwrap()
            .id
            .clone();
        let top = snap
            .tree
            .iter()
            .find(|n| n.kind == "top_event")
            .unwrap()
            .id
            .clone();
        let snap = s
            .add_entity(
                &top,
                "threat",
                "Hose damage",
                "Line one\n日本語",
                &json!({"code":"T1"}),
            )
            .unwrap();
        let threat = snap
            .tree
            .iter()
            .find(|n| n.kind == "threat")
            .unwrap()
            .id
            .clone();
        let lookup = crate::risk::Lookup {
            id: "rating".into(),
            table_key: "effectiveness".into(),
            code: "E1".into(),
            title: "Effective".into(),
            color: "#00aa00".into(),
            sort_order: 0,
            metadata: json!({"description":"Checked","frequency":0.1}),
        };
        s.save_lookup(&lookup).unwrap();
        s.custom_field("barrier", "note", "text", None, false)
            .unwrap();
        let snap = s
            .add_entity(
                &threat,
                "barrier",
                "Inspection",
                "Before transfer",
                &json!({"effectiveness_id":"rating","user_data":{"note":"Recorded"}}),
            )
            .unwrap();
        let barrier = snap
            .tree
            .iter()
            .find(|n| n.kind == "barrier")
            .unwrap()
            .id
            .clone();
        s.set_positions(
            &diagram,
            &[crate::editing::Position {
                id: barrier.clone(),
                x: 245.5,
                y: 600.0,
            }],
        )
        .unwrap();
        s.add_entity(
            &barrier,
            "action",
            "Review schedule",
            "Assigned work",
            &json!({}),
        )
        .unwrap();
        let activity = s
            .add_entity(
                &root,
                "activity",
                "Inspect hose",
                "Work instruction",
                &json!({}),
            )
            .unwrap()
            .tree
            .into_iter()
            .find(|n| n.kind == "activity")
            .unwrap()
            .id;
        s.link(&barrier, &activity, "activity", false).unwrap();
        let matrix = crate::risk::Matrix {
            id: "matrix".into(),
            title: "Transfer risk".into(),
            coordinate_order: "yx".into(),
            likelihood: vec![crate::risk::Axis {
                code: "L1".into(),
                title: "Rare".into(),
            }],
            severity: vec![crate::risk::Axis {
                code: "S1".into(),
                title: "Minor".into(),
            }],
            categories: vec![crate::risk::Category {
                id: "low".into(),
                title: "Low".into(),
                color: "#00aa00".into(),
                rank: 0,
            }],
            cells: vec![crate::risk::Cell {
                likelihood_position: 0,
                severity_position: 0,
                category_id: "low".into(),
                label: "Review".into(),
            }],
        };
        s.save_matrix(&matrix).unwrap();
        s.set_assessment(
            &crate::risk::Assessment {
                target_id: top,
                matrix_id: "matrix".into(),
                phase: "inherent".into(),
                likelihood_position: 0,
                severity_position: 0,
            },
            false,
        )
        .unwrap();
        s
    }
    #[test]
    fn export_import_export_structural_equality_and_save_reopen() {
        let source = populated();
        let exported = source.table_export().unwrap().text;
        let mut target = CaseStore::default();
        target.create("Empty", "New organization").unwrap();
        let preview = target.import_preview(&exported).unwrap();
        assert!(preview.errors.is_empty(), "{:?}", preview.errors);
        assert_eq!(preview.counts["action"], 1);
        assert_eq!(preview.counts["matrix"], 1);
        target.import_apply(&exported, &preview.signature).unwrap();
        assert_eq!(target.table_export().unwrap().text, exported);
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("roundtrip.sqlite");
        target.save(Some(&file)).unwrap();
        target.close();
        target.open(&file).unwrap();
        assert_eq!(target.table_export().unwrap().text, exported);
        let p = target.import_preview(&exported).unwrap();
        assert!(p
            .errors
            .iter()
            .any(|e| e.message.contains("Duplicate existing entity")));
    }
    #[test]
    fn rejection_bad_parent_cycle_duplicate_kind_and_csv_preserves_session_and_bytes() {
        let mut source = populated();
        let exported = source.table_export().unwrap().text;
        let base = decode(&exported)
            .unwrap()
            .into_iter()
            .map(|(_, r)| r)
            .collect::<Vec<_>>();
        let entity = base
            .iter()
            .position(|r| r[0] == "entity" && r[3] == "group")
            .unwrap();
        let mut bad_parent = base.clone();
        bad_parent[entity][2] = "missing".into();
        let mut cycle = base.clone();
        cycle[entity][2] = cycle[entity][1].clone();
        let mut duplicate = base.clone();
        duplicate.push(base[entity].clone());
        let mut kind = base.clone();
        kind[entity][3] = "unknown".into();
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("existing.sqlite");
        source.save(Some(&file)).unwrap();
        let before = serde_json::to_value(source.snapshot().unwrap()).unwrap();
        let bytes = std::fs::read(&file).unwrap();
        for (text, expected_error) in [
            (encode(&bad_parent), "Missing parent"),
            (encode(&cycle), "Hierarchy cycle"),
            (encode(&duplicate), "Duplicate record identity"),
            (encode(&kind), "Unknown entity kind"),
            ("record_type,\"unclosed".into(), "Unclosed quoted field"),
        ] {
            let p = source.import_preview(&text).unwrap();
            assert!(!p.errors.is_empty());
            assert!(
                p.errors.iter().any(|e| e.message.contains(expected_error)),
                "{:?}",
                p.errors
            );
            assert!(p.errors.iter().all(|e| e.line >= 1));
            assert!(source.import_apply(&text, &p.signature).is_err());
            assert_eq!(
                serde_json::to_value(source.snapshot().unwrap()).unwrap(),
                before
            );
            assert_eq!(std::fs::read(&file).unwrap(), bytes);
        }
        // On an empty target, a late invalid typed property follows successful inserts
        // in the scratch transaction and must still leave zero partial creations.
        let mut invalid_property = base.clone();
        let barrier = base
            .iter()
            .position(|r| r[0] == "entity" && r[3] == "barrier")
            .unwrap();
        let mut payload: Value = serde_json::from_str(&invalid_property[barrier][7]).unwrap();
        payload["properties"]["unknown_field"] = json!("bad");
        invalid_property[barrier][7] = payload.to_string();
        let mut empty = CaseStore::default();
        empty.create("Empty", "Workshop").unwrap();
        let before = serde_json::to_value(empty.snapshot().unwrap()).unwrap();
        let text = encode(&invalid_property);
        let p = empty.import_preview(&text).unwrap();
        let physical_line = decode(&text).unwrap()[barrier].0;
        assert!(
            p.errors
                .iter()
                .any(|e| e.line == physical_line && e.message.contains("Unsupported property")),
            "{:?}",
            p.errors
        );
        assert!(empty.import_apply(&text, &p.signature).is_err());
        assert_eq!(
            serde_json::to_value(empty.snapshot().unwrap()).unwrap(),
            before
        );
    }
    #[test]
    fn stale_import_and_csv_line_diagnostics() {
        let source = populated();
        let text = source.table_export().unwrap().text;
        let mut target = CaseStore::default();
        let root = target.create("Empty", "Workshop").unwrap().tree[0]
            .id
            .clone();
        let p = target.import_preview(&text).unwrap();
        target
            .edit_entity(&root, "Changed", "", &json!({}))
            .unwrap();
        assert!(target.import_apply(&text, &p.signature).is_err());
        let p = target.import_preview(&text).unwrap();
        assert!(target
            .import_apply(&(text.clone() + "bad\n"), &p.signature)
            .is_err());
        let multiline = "a,b\r\n\"line\none\",ok\r\nmissing\r\n";
        let rows = decode(multiline).unwrap();
        assert_eq!(rows[2].0, 4);
        assert_eq!(rows[1].1[0], "line\none");
        assert!(decode("a,\"b\"unexpected\n").is_err());
        assert!(decode("a,b\"c\n").is_err());
    }
    #[test]
    fn every_case_header_is_validated_and_persisted_whitespace_round_trips() {
        let source = populated();
        let db = &source.active.as_ref().unwrap().db;
        db.execute("UPDATE entity SET title=' Station ' WHERE kind='group'", [])
            .unwrap();
        db.execute(
            "UPDATE lookup_value SET title=' Effective ' WHERE id='rating'",
            [],
        )
        .unwrap();
        db.execute(
            "UPDATE risk_matrix SET title=' Transfer risk ' WHERE id='matrix'",
            [],
        )
        .unwrap();
        let text = source.table_export().unwrap().text;
        let mut target = CaseStore::default();
        target.create("New", "Organization").unwrap();
        let p = target.import_preview(&text).unwrap();
        assert!(p.errors.is_empty(), "{:?}", p.errors);
        target.import_apply(&text, &p.signature).unwrap();
        assert_eq!(target.table_export().unwrap().text, text);
        let mut rows = decode(&text)
            .unwrap()
            .into_iter()
            .map(|(_, r)| r)
            .collect::<Vec<_>>();
        let mut extra = rows[1].clone();
        extra[1] = "unexpected-case".into();
        rows.push(extra);
        assert!(target
            .import_preview(&encode(&rows))
            .unwrap()
            .errors
            .iter()
            .any(|e| e.message.contains("Exactly one case")));
        rows.pop();
        rows[1][7]=json!({"author":25,"message":false,"metadata":{"application":"other","format_version":99}}).to_string();
        assert!(target
            .import_preview(&encode(&rows))
            .unwrap()
            .errors
            .iter()
            .any(|e| e.line == 2 && e.message.contains("case header payload")));
    }
    #[test]
    fn lookup_reorder_is_atomic_and_preserves_reference_identities() {
        let mut store = populated();
        store
            .save_lookup(&crate::risk::Lookup {
                id: "other".into(),
                table_key: "effectiveness".into(),
                code: "E2".into(),
                title: "Other".into(),
                color: "#aaaaaa".into(),
                sort_order: 10,
                metadata: json!({}),
            })
            .unwrap();
        let before = serde_json::to_value(&store.snapshot().unwrap().unwrap().tree).unwrap();
        let reordered = store.reorder_lookup("other", -1).unwrap();
        assert_eq!(
            reordered
                .lookups
                .iter()
                .map(|l| (&l.id, l.sort_order))
                .collect::<Vec<_>>(),
            vec![(&"other".to_string(), 0), (&"rating".to_string(), 1)]
        );
        assert_eq!(serde_json::to_value(reordered.tree).unwrap(), before);
        let unchanged = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store.reorder_lookup("other", -1).is_err());
        assert!(store.reorder_lookup("other", 0).is_err());
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            unchanged
        );
    }
}
