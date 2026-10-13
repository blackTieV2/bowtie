use crate::case::{CaseSnapshot, CaseStore};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
type Result<T> = std::result::Result<T, String>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Axis {
    pub code: String,
    pub title: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub color: String,
    pub rank: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub likelihood_position: i64,
    pub severity_position: i64,
    pub category_id: String,
    pub label: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    pub id: String,
    pub title: String,
    pub coordinate_order: String,
    pub likelihood: Vec<Axis>,
    pub severity: Vec<Axis>,
    pub categories: Vec<Category>,
    pub cells: Vec<Cell>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessment {
    pub target_id: String,
    pub matrix_id: String,
    pub phase: String,
    pub likelihood_position: i64,
    pub severity_position: i64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lookup {
    pub id: String,
    pub table_key: String,
    pub code: String,
    pub title: String,
    pub color: String,
    pub sort_order: i64,
    pub metadata: Value,
}
pub fn matrices(db: &Connection) -> Result<Vec<Matrix>> {
    let mut q=db.prepare("SELECT id,CASE WHEN coalesce(json_extract(metadata_json,'$.hidden'),0)=1 THEN '' ELSE title END,coordinate_order FROM risk_matrix ORDER BY id").map_err(err)?;
    let basics = q
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })
        .map_err(err)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(err)?;
    let mut out = vec![];
    for (id, title, coordinate_order) in basics {
        let axis = |name: &str| -> Result<Vec<Axis>> {
            let mut q=db.prepare("SELECT code,title FROM risk_axis WHERE matrix_id=? AND axis=? ORDER BY position").map_err(err)?;
            let rows = q
                .query_map(params![id, name], |r| {
                    Ok(Axis {
                        code: r.get(0)?,
                        title: r.get(1)?,
                    })
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            Ok(rows)
        };
        let likelihood = axis("likelihood")?;
        let severity = axis("severity")?;
        let categories = {
            let mut q = db
                .prepare(
                    "SELECT id,title,color,rank FROM risk_category WHERE matrix_id=? ORDER BY rank",
                )
                .map_err(err)?;
            let rows = q
                .query_map([&id], |r| {
                    Ok(Category {
                        id: r.get(0)?,
                        title: r.get(1)?,
                        color: r.get(2)?,
                        rank: r.get(3)?,
                    })
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            rows
        };
        let cells = {
            let mut q=db.prepare("SELECT likelihood_position,severity_position,category_id,label FROM risk_cell WHERE matrix_id=? ORDER BY severity_position,likelihood_position").map_err(err)?;
            let rows = q
                .query_map([&id], |r| {
                    Ok(Cell {
                        likelihood_position: r.get(0)?,
                        severity_position: r.get(1)?,
                        category_id: r.get(2)?,
                        label: r.get(3)?,
                    })
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            rows
        };
        out.push(Matrix {
            id,
            title,
            coordinate_order,
            likelihood,
            severity,
            categories,
            cells,
        });
    }
    Ok(out)
}
pub fn assessments(db: &Connection) -> Result<Vec<Assessment>> {
    let mut q=db.prepare("SELECT target_id,matrix_id,phase,likelihood_position,severity_position FROM risk_assessment ORDER BY target_id,matrix_id,phase").map_err(err)?;
    let rows = q
        .query_map([], |r| {
            Ok(Assessment {
                target_id: r.get(0)?,
                matrix_id: r.get(1)?,
                phase: r.get(2)?,
                likelihood_position: r.get(3)?,
                severity_position: r.get(4)?,
            })
        })
        .map_err(err)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(err)?;
    Ok(rows)
}
pub fn lookups(db: &Connection) -> Result<Vec<Lookup>> {
    let mut q=db.prepare("SELECT id,table_key,code,title,coalesce(color,'#d9e9d9'),sort_order,metadata_json FROM lookup_value ORDER BY table_key,sort_order,id").map_err(err)?;
    let rows = q
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, i64>(5)?,
                r.get::<_, String>(6)?,
            ))
        })
        .map_err(err)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(err)?;
    rows.into_iter()
        .map(|(id, table_key, code, title, color, sort_order, m)| {
            Ok(Lookup {
                id,
                table_key,
                code,
                title,
                color,
                sort_order,
                metadata: serde_json::from_str(&m).map_err(err)?,
            })
        })
        .collect()
}
fn color_valid(c: &str) -> bool {
    c.len() == 7 && c.starts_with('#') && c[1..].bytes().all(|b| b.is_ascii_hexdigit())
}
impl CaseStore {
    pub fn reorder_lookup(&mut self, id: &str, direction: i64) -> Result<CaseSnapshot> {
        if ![-1, 1].contains(&direction) {
            return Err("Choose the previous or next position".into());
        }
        let values = lookups(&self.active.as_ref().ok_or("No case is open")?.db)?;
        let table = &values
            .iter()
            .find(|l| l.id == id)
            .ok_or("Choice no longer exists")?
            .table_key;
        let mut values = values
            .iter()
            .filter(|l| &l.table_key == table)
            .collect::<Vec<_>>();
        let index = values.iter().position(|l| l.id == id).unwrap();
        let next = index as i64 + direction;
        if next < 0 || next >= values.len() as i64 {
            return Err("Choice is already at the end of the list".into());
        }
        values.swap(index, next as usize);
        let tx = self
            .active
            .as_mut()
            .unwrap()
            .db
            .transaction()
            .map_err(err)?;
        for (order, value) in values.iter().enumerate() {
            tx.execute(
                "UPDATE lookup_value SET sort_order=? WHERE id=?",
                params![order as i64, value.id],
            )
            .map_err(err)?;
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn save_matrix(&mut self, m: &Matrix) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        write_matrix(&tx, m)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn delete_matrix(&mut self, id: &str) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let count: i64 = tx
            .query_row(
                "SELECT count(*) FROM risk_assessment WHERE matrix_id=?",
                [id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if count > 0 {
            return Err(format!("Cannot delete matrix: {count} assessment(s) reference it. Clear assessments or hide its name instead."));
        }
        for table in ["risk_cell", "risk_category", "risk_axis", "risk_matrix"] {
            let column = if table == "risk_matrix" {
                "id"
            } else {
                "matrix_id"
            };
            tx.execute(&format!("DELETE FROM {table} WHERE {column}=?"), [id])
                .map_err(err)?;
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn set_assessment(&mut self, a: &Assessment, remove: bool) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        if remove {
            tx.execute(
                "DELETE FROM risk_assessment WHERE target_id=? AND matrix_id=? AND phase=?",
                params![a.target_id, a.matrix_id, a.phase],
            )
            .map_err(err)?;
        } else {
            tx.execute("INSERT INTO risk_assessment VALUES(?,?,?,?,?) ON CONFLICT(target_id,matrix_id,phase) DO UPDATE SET likelihood_position=excluded.likelihood_position,severity_position=excluded.severity_position",params![a.target_id,a.matrix_id,a.phase,a.likelihood_position,a.severity_position]).map_err(err)?;
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn clear_assessments(&mut self) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        tx.execute("DELETE FROM risk_assessment", []).map_err(err)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn save_lookup(&mut self, l: &Lookup) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        write_lookup(&tx, l)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn delete_lookup(&mut self, id: &str) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let count: i64 = tx
            .query_row(
                "SELECT count(*) FROM entity_property WHERE json_extract(value_json,'$')=?",
                [id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if count > 0 {
            return Err(format!(
                "Cannot delete taxonomy: {count} property reference(s). Unassign them first."
            ));
        }
        tx.execute("DELETE FROM lookup_value WHERE id=?", [id])
            .map_err(|_| {
                "Cannot delete taxonomy: effectiveness references remain. Unassign them first."
                    .to_string()
            })?;
        tx.commit().map_err(err)?;
        self.edited()
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRow {
    pub id: String,
    pub parent_id: String,
    pub parent_title: String,
    pub title: String,
    pub description: String,
    pub properties: Value,
    pub responsible: Vec<String>,
}
pub fn action_register(db: &Connection) -> Result<Vec<ActionRow>> {
    let mut q=db.prepare("SELECT a.id,a.parent_id,p.title,a.title,a.description,coalesce((SELECT json_group_object(property_key,json(value_json)) FROM entity_property WHERE entity_id=a.id),'{}'),coalesce((SELECT json_group_array(t.title) FROM entity_link l JOIN entity t ON t.id=l.target_id WHERE l.source_id=a.id AND l.relation='action_party'),'[]') FROM entity a JOIN entity p ON p.id=a.parent_id WHERE a.kind='action' ORDER BY a.title,a.id").map_err(err)?;
    let rows = q
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })
        .map_err(err)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(err)?;
    rows.into_iter()
        .map(|(id, parent_id, parent_title, title, description, p, r)| {
            Ok(ActionRow {
                id,
                parent_id,
                parent_title,
                title,
                description,
                properties: serde_json::from_str(&p).map_err(err)?,
                responsible: serde_json::from_str(&r).map_err(err)?,
            })
        })
        .collect()
}

pub(crate) fn write_matrix(tx: &rusqlite::Transaction<'_>, m: &Matrix) -> Result<()> {
    if m.likelihood.is_empty() || m.severity.is_empty() || m.categories.is_empty() {
        return Err("Provide both axes and at least one category".into());
    }
    if m.categories
        .iter()
        .any(|c| c.title.trim().is_empty() || !color_valid(&c.color))
        || m.likelihood
            .iter()
            .chain(m.severity.iter())
            .any(|a| a.code.trim().is_empty() || a.title.trim().is_empty())
    {
        return Err("Axis labels/category names and valid colors are required".into());
    }
    let ranks = m
        .categories
        .iter()
        .map(|c| c.rank)
        .collect::<std::collections::HashSet<_>>();
    if ranks.len() != m.categories.len()
        || (0..m.categories.len() as i64).any(|r| !ranks.contains(&r))
        || m.categories
            .iter()
            .map(|c| &c.id)
            .collect::<std::collections::HashSet<_>>()
            .len()
            != m.categories.len()
    {
        return Err("Categories must have contiguous low-to-high ranks".into());
    }
    let keys = m
        .cells
        .iter()
        .map(|c| (c.likelihood_position, c.severity_position))
        .collect::<std::collections::HashSet<_>>();
    if keys.len() != m.likelihood.len() * m.severity.len()
        || keys.len() != m.cells.len()
        || m.cells.iter().any(|c| {
            c.likelihood_position < 0
                || c.likelihood_position >= m.likelihood.len() as i64
                || c.severity_position < 0
                || c.severity_position >= m.severity.len() as i64
                || !m.categories.iter().any(|cat| cat.id == c.category_id)
        })
    {
        return Err("Every matrix cell needs exactly one valid category".into());
    }

    let existing: bool = tx
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM risk_matrix WHERE id=?)",
            [&m.id],
            |r| r.get(0),
        )
        .map_err(err)?;
    if !existing && m.title.trim().is_empty() {
        return Err("Name a new matrix before hiding it".into());
    }
    if existing {
        tx.execute("UPDATE risk_matrix SET title=CASE WHEN trim(?)='' THEN title ELSE ? END,coordinate_order=?,metadata_json=json_set(metadata_json,'$.hidden',json(?)) WHERE id=?",params![m.title,m.title.trim(),m.coordinate_order,if m.title.trim().is_empty(){"true"}else{"false"},m.id]).map_err(err)?;
    } else {
        tx.execute(
            "INSERT INTO risk_matrix(id,title,coordinate_order) VALUES(?,?,?)",
            params![m.id, m.title.trim(), m.coordinate_order],
        )
        .map_err(err)?;
    }
    // Upsert retained rows first. Only obsolete rows are deleted; existing
    // assessments therefore remain valid, and RESTRICT blocks unsafe resizing.
    for (axis, levels) in [("likelihood", &m.likelihood), ("severity", &m.severity)] {
        for (i, a) in levels.iter().enumerate() {
            tx.execute("INSERT INTO risk_axis VALUES(?,?,?,?,?) ON CONFLICT(matrix_id,axis,position) DO UPDATE SET code=excluded.code,title=excluded.title",params![m.id,axis,i as i64,a.code,a.title]).map_err(err)?;
        }
    }
    // Free unique ranks transactionally before applying the new order.
    let max_rank: i64 = tx
        .query_row(
            "SELECT coalesce(max(rank),0)+? FROM risk_category WHERE matrix_id=?",
            params![m.categories.len() as i64 + 1, m.id],
            |r| r.get(0),
        )
        .map_err(err)?;
    tx.execute(
        "UPDATE risk_category SET rank=rank+? WHERE matrix_id=?",
        params![max_rank, m.id],
    )
    .map_err(err)?;
    for c in &m.categories {
        tx.execute("INSERT INTO risk_category VALUES(?,?,?,?,?) ON CONFLICT(matrix_id,id) DO UPDATE SET title=excluded.title,color=excluded.color,rank=excluded.rank",params![m.id,c.id,c.title.trim(),c.color,c.rank]).map_err(err)?;
    }
    for c in &m.cells {
        tx.execute("INSERT INTO risk_cell(matrix_id,likelihood_position,severity_position,category_id,label) VALUES(?,?,?,?,?) ON CONFLICT(matrix_id,likelihood_position,severity_position) DO UPDATE SET category_id=excluded.category_id,label=excluded.label",params![m.id,c.likelihood_position,c.severity_position,c.category_id,c.label]).map_err(err)?;
    }
    tx.execute("DELETE FROM risk_cell WHERE matrix_id=? AND (likelihood_position>=? OR severity_position>=?)",params![m.id,m.likelihood.len() as i64,m.severity.len() as i64]).map_err(|_|"Cannot remove a matrix level referenced by assessments. Clear those assessments first.".to_string())?;
    for (axis, count) in [
        ("likelihood", m.likelihood.len()),
        ("severity", m.severity.len()),
    ] {
        tx.execute(
            "DELETE FROM risk_axis WHERE matrix_id=? AND axis=? AND position>=?",
            params![m.id, axis, count as i64],
        )
        .map_err(err)?;
    }
    let old_ids = {
        let mut q = tx
            .prepare("SELECT id FROM risk_category WHERE matrix_id=?")
            .map_err(err)?;
        let r = q
            .query_map([&m.id], |r| r.get::<_, String>(0))
            .map_err(err)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(err)?;
        r
    };
    for id in old_ids {
        if !m.categories.iter().any(|c| c.id == id) {
            tx.execute(
                "DELETE FROM risk_category WHERE matrix_id=? AND id=?",
                params![m.id, id],
            )
            .map_err(err)?;
        }
    }

    Ok(())
}

pub(crate) fn write_lookup(tx: &rusqlite::Transaction<'_>, l: &Lookup) -> Result<()> {
    let catalog = crate::properties::catalog();
    let tables = catalog
        .as_object()
        .unwrap()
        .values()
        .filter_map(Value::as_array)
        .flatten()
        .filter_map(|f| f["table"].as_str())
        .collect::<Vec<_>>();
    if !tables.contains(&l.table_key.as_str())
        || !color_valid(&l.color)
        || l.title.trim().is_empty()
        || l.sort_order < 0
        || !l.metadata.is_object()
        || l.metadata
            .get("description")
            .is_some_and(|v| !v.is_string())
        || l.metadata
            .get("frequency")
            .is_some_and(|v| !v.is_null() && !v.as_f64().is_some_and(|n| n.is_finite() && n >= 0.0))
    {
        return Err("Invalid taxonomy value".into());
    }

    if let Some(values) = l.metadata.get("user_data") {
        crate::properties::validate_custom(&tx, &format!("lookup:{}", l.table_key), values)?;
    }
    tx.execute("INSERT INTO lookup_value(id,table_key,code,title,color,sort_order,metadata_json) VALUES(?,?,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET code=excluded.code,title=excluded.title,color=excluded.color,sort_order=excluded.sort_order,metadata_json=excluded.metadata_json",params![l.id,l.table_key,l.code,l.title.trim(),l.color,l.sort_order,l.metadata.to_string()]).map_err(err)?;
    let table: String = tx
        .query_row(
            "SELECT table_key FROM lookup_value WHERE id=?",
            [&l.id],
            |r| r.get(0),
        )
        .map_err(err)?;
    if table != l.table_key {
        return Err("Taxonomy identity is immutable".into());
    }

    Ok(())
}
