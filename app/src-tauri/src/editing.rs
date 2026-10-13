//! Phase 2 typed editing. All writes use the approved schema and its triggers.
use crate::case::{CaseSnapshot, CaseStore};
use rusqlite::{params, Connection, Transaction};
use serde::Deserialize;
use serde_json::Value;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[derive(Deserialize)]
pub struct Position {
    pub id: String,
    pub x: f64,
    pub y: f64,
}
const CANVAS: &[&str] = &[
    "hazard",
    "top_event",
    "threat",
    "consequence",
    "barrier",
    "escalation_factor",
];
fn properties(tx: &Transaction<'_>, id: &str, values: &Value) -> Result<()> {
    crate::properties::write(tx, id, values)
}
fn belongs(db: &Connection, id: &str, diagram: &str) -> Result<bool> {
    db.query_row("WITH RECURSIVE a(id,parent_id) AS (SELECT id,parent_id FROM entity WHERE id=? UNION SELECT e.id,e.parent_id FROM entity e JOIN a ON e.id=a.parent_id) SELECT EXISTS(SELECT 1 FROM a WHERE id=?)", params![id, diagram], |r| r.get(0)).map_err(err)
}
impl CaseStore {
    pub(crate) fn edited(&mut self) -> Result<CaseSnapshot> {
        self.active.as_mut().ok_or("No case is open")?.dirty = true;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
    pub fn add_entity(
        &mut self,
        parent: &str,
        kind: &str,
        title: &str,
        description: &str,
        values: &Value,
    ) -> Result<CaseSnapshot> {
        if ![
            "threat",
            "consequence",
            "barrier",
            "escalation_factor",
            "activity",
            "document",
            "system",
            "competency",
            "job_title",
            "product",
            "action",
        ]
        .contains(&kind)
        {
            return Err("Use the case/group/diagram commands for structural roots".into());
        }
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let id = uuid::Uuid::new_v4().to_string();
        let order: i64 = tx
            .query_row(
                "SELECT coalesce(max(sort_order),-1)+1 FROM entity WHERE parent_id=? AND kind=?",
                params![parent, kind],
                |r| r.get(0),
            )
            .map_err(err)?;
        insert_entity(&tx, &id, parent, kind, title.trim(), description, order)?;
        properties(&tx, &id, values)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn edit_entity(
        &mut self,
        id: &str,
        title: &str,
        description: &str,
        values: &Value,
    ) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        if tx
            .execute(
                "UPDATE entity SET title=?,description=? WHERE id=?",
                params![title.trim(), description, id],
            )
            .map_err(err)?
            != 1
        {
            return Err("Entity no longer exists".into());
        }
        properties(&tx, id, values)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn link(
        &mut self,
        source: &str,
        target: &str,
        relation: &str,
        remove: bool,
    ) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        if remove {
            tx.execute(
                "DELETE FROM entity_link WHERE source_id=? AND target_id=? AND relation=?",
                params![source, target, relation],
            )
            .map_err(err)?;
        } else {
            tx.execute(
                "INSERT INTO entity_link(source_id,target_id,relation) VALUES(?,?,?)",
                params![source, target, relation],
            )
            .map_err(err)?;
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn set_positions(&mut self, diagram: &str, positions: &[Position]) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let kind: String = tx
            .query_row("SELECT kind FROM entity WHERE id=?", [diagram], |r| {
                r.get(0)
            })
            .map_err(err)?;
        if kind != "diagram" {
            return Err("Select a diagram".into());
        }
        for p in positions {
            if !p.x.is_finite()
                || !p.y.is_finite()
                || p.x.abs() > 1_000_000.0
                || p.y.abs() > 1_000_000.0
            {
                return Err("Position must be finite and within the canvas range".into());
            }
            let kind: String = tx
                .query_row("SELECT kind FROM entity WHERE id=?", [&p.id], |r| r.get(0))
                .map_err(err)?;
            let diagram_hazard: String = tx
                .query_row("SELECT parent_id FROM entity WHERE id=?", [diagram], |r| {
                    r.get(0)
                })
                .map_err(err)?;
            if !CANVAS.contains(&kind.as_str())
                || !(belongs(&tx, &p.id, diagram)? || (kind == "hazard" && p.id == diagram_hazard))
            {
                return Err("Position target is outside this diagram".into());
            }
            let value = serde_json::json!({"x":p.x,"y":p.y}).to_string();
            tx.execute("INSERT INTO entity_property(entity_id,property_key,value_json) VALUES(?,'canvas.position',?) ON CONFLICT(entity_id,property_key) DO UPDATE SET value_json=excluded.value_json", params![p.id,value]).map_err(err)?;
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn delete_entity(&mut self, id: &str) -> Result<CaseSnapshot> {
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let kind: String = tx
            .query_row("SELECT kind FROM entity WHERE id=?", [id], |r| r.get(0))
            .map_err(err)?;
        if kind == "organization" || kind == "top_event" {
            return Err("The organization and a diagram's sole top event are required. Edit their names, or delete the empty diagram through the explorer.".into());
        }
        if kind == "diagram" {
            let top: String = tx
                .query_row(
                    "SELECT id FROM entity WHERE parent_id=? AND kind='top_event'",
                    [id],
                    |r| r.get(0),
                )
                .map_err(err)?;
            let blockers = deletion_references(&tx, &top)?;
            if !blockers.is_empty() {
                return Err(format!(
                    "Cannot delete diagram: its top event has references (RESTRICT): {}",
                    blockers.join("; ")
                ));
            }
            // Diagram and its required top event are one structural unit.
            tx.execute("DELETE FROM entity_property WHERE entity_id=?", [&top])
                .map_err(err)?;
            tx.execute("DELETE FROM entity WHERE id=?", [&top])
                .map_err(err)?;
        }
        let blockers = deletion_references(&tx, id)?;
        if !blockers.is_empty() {
            return Err(format!("Cannot delete: references block removal (RESTRICT). Remove these references or children first: {}", blockers.join("; ")));
        }
        // Explicitly remove owned property rows; shared links/children are never cascaded.
        tx.execute("DELETE FROM entity_property WHERE entity_id=?", [id])
            .map_err(err)?;
        tx.execute("DELETE FROM entity WHERE id=?", [id])
            .map_err(err)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
}
pub(crate) fn insert_entity(
    tx: &Transaction<'_>,
    id: &str,
    parent: &str,
    kind: &str,
    title: &str,
    description: &str,
    order: i64,
) -> Result<()> {
    tx.execute(
        "INSERT INTO entity(id,parent_id,kind,title,description,sort_order) VALUES(?,?,?,?,?,?)",
        params![id, parent, kind, title, description, order],
    )
    .map_err(err)?;
    Ok(())
}
fn deletion_references(db: &Connection, id: &str) -> Result<Vec<String>> {
    let mut blockers = vec![];
    let mut equivalents = db.prepare("SELECT e.title FROM entity_property p JOIN entity e ON e.id=p.entity_id WHERE p.property_key='equivalent_top_event' AND json_extract(p.value_json,'$')=?").map_err(err)?;
    for row in equivalents
        .query_map([id], |r| r.get::<_, String>(0))
        .map_err(err)?
    {
        blockers.push(format!(
            "equivalent top-event reference: {}",
            row.map_err(err)?
        ));
    }
    let mut children = db
        .prepare("SELECT kind,title FROM entity WHERE parent_id=? ORDER BY kind,sort_order,id")
        .map_err(err)?;
    for row in children
        .query_map([id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
        })
        .map_err(err)?
    {
        let (kind, title) = row.map_err(err)?;
        blockers.push(format!("child {kind}: {title}"));
    }
    let mut links = db.prepare("SELECT l.relation,e.title,CASE WHEN l.source_id=? THEN 'outgoing' ELSE 'incoming' END FROM entity_link l JOIN entity e ON e.id=CASE WHEN l.source_id=? THEN l.target_id ELSE l.source_id END WHERE l.source_id=? OR l.target_id=?").map_err(err)?;
    for row in links
        .query_map(params![id, id, id, id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })
        .map_err(err)?
    {
        let (relation, title, direction) = row.map_err(err)?;
        blockers.push(format!("{direction} {relation} link: {title}"));
    }
    // All remaining entity FK tables in the approved schema, including future modules.
    for (table, column) in [
        ("risk_assessment", "target_id"),
        ("audit_question_target", "target_id"),
        ("incident_barrier_link", "barrier_id"),
    ] {
        let count: i64 = db
            .query_row(
                &format!("SELECT count(*) FROM {table} WHERE {column}=?"),
                [id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if count > 0 {
            blockers.push(format!("{table}.{column}: {count} reference(s)"));
        }
    }
    Ok(blockers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn fixture() -> (CaseStore, String, String, String) {
        let mut store = CaseStore::default();
        let root = store.create("Canvas", "Workshop").unwrap().tree[0]
            .id
            .clone();
        let group = store
            .add_group(&root, "Transfer")
            .unwrap()
            .tree
            .into_iter()
            .find(|n| n.kind == "group")
            .unwrap()
            .id;
        let snapshot = store
            .add_diagram(&group, "Transfer liquid", "Containment lost")
            .unwrap();
        let diagram = snapshot
            .tree
            .iter()
            .find(|n| n.kind == "diagram")
            .unwrap()
            .id
            .clone();
        let top = snapshot
            .tree
            .iter()
            .find(|n| n.kind == "top_event")
            .unwrap()
            .id
            .clone();
        (store, root, diagram, top)
    }
    fn add(store: &mut CaseStore, parent: &str, kind: &str, title: &str) -> String {
        store
            .add_entity(parent, kind, title, "", &json!({}))
            .unwrap()
            .tree
            .into_iter()
            .find(|n| n.title == title)
            .unwrap()
            .id
    }
    #[test]
    fn edits_and_positions_round_trip_without_schema_change() {
        let (mut store, root, diagram, top) = fixture();
        let threat = add(&mut store, &top, "threat", "Hose fails");
        let barrier = add(&mut store, &threat, "barrier", "Inspection");
        let task = add(&mut store, &root, "activity", "Inspect hose");
        store
            .edit_entity(
                &task,
                "Inspect hose",
                "Recurring",
                &json!({"timing":"Before every transfer","verification":"Signed checklist"}),
            )
            .unwrap();
        store.link(&barrier, &task, "activity", false).unwrap();
        store
            .set_positions(
                &diagram,
                &[Position {
                    id: barrier.clone(),
                    x: 311.5,
                    y: -48.25,
                }],
            )
            .unwrap();
        let before = serde_json::to_value(store.snapshot().unwrap().unwrap()).unwrap();
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("canvas.sqlite");
        store.save(Some(&path)).unwrap();
        store.close();
        let after = serde_json::to_value(store.open(&path).unwrap()).unwrap();
        assert_eq!(before["tree"], after["tree"]);
        assert_eq!(before["links"], after["links"]);
        assert_eq!(after["links"].as_array().unwrap().len(), 1);
    }
    #[test]
    fn typed_parent_and_link_triggers_remain_authoritative() {
        let (mut store, root, _, top) = fixture();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store
            .add_entity(&top, "barrier", "Invalid", "", &json!({}))
            .unwrap_err()
            .contains("Invalid parent"));
        assert!(store
            .link(&top, &root, "activity", false)
            .unwrap_err()
            .contains("Invalid"));
        assert_eq!(
            before,
            serde_json::to_value(store.snapshot().unwrap()).unwrap()
        );
    }
    #[test]
    fn batch_positions_are_atomic_and_diagram_scoped() {
        let (mut store, root, diagram, top) = fixture();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store
            .set_positions(
                &diagram,
                &[
                    Position {
                        id: top.clone(),
                        x: 100.,
                        y: 200.
                    },
                    Position {
                        id: root,
                        x: 0.,
                        y: 0.
                    }
                ]
            )
            .is_err());
        assert_eq!(
            before,
            serde_json::to_value(store.snapshot().unwrap()).unwrap()
        );
        assert!(store
            .set_positions(
                &diagram,
                &[Position {
                    id: top,
                    x: f64::NAN,
                    y: 0.
                }]
            )
            .is_err());
    }
    #[test]
    fn restrict_names_children_and_links_and_preserves_dirty_state() {
        let (mut store, root, diagram, top) = fixture();
        let threat = add(&mut store, &top, "threat", "Hose rupture");
        let barrier = add(&mut store, &threat, "barrier", "Pressure limit");
        let task = add(&mut store, &root, "activity", "Check limiter");
        store.link(&barrier, &task, "activity", false).unwrap();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store
            .delete_entity(&threat)
            .unwrap_err()
            .contains("Pressure limit"));
        assert!(store
            .delete_entity(&task)
            .unwrap_err()
            .contains("Pressure limit"));
        assert!(store
            .delete_entity(&barrier)
            .unwrap_err()
            .contains("Check limiter"));
        assert!(store
            .delete_entity(&diagram)
            .unwrap_err()
            .contains("Hose rupture"));
        assert_eq!(
            before,
            serde_json::to_value(store.snapshot().unwrap()).unwrap()
        );
        store.link(&barrier, &task, "activity", true).unwrap();
        store.delete_entity(&barrier).unwrap();
        store.delete_entity(&threat).unwrap();
        store.delete_entity(&diagram).unwrap();
        let folder = tempfile::tempdir().unwrap();
        store
            .save(Some(&folder.path().join("deleted.sqlite")))
            .unwrap();
    }
    #[test]
    fn invalid_property_edit_rolls_back_name_and_content() {
        let (mut store, _, _, top) = fixture();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store
            .edit_entity(
                &top,
                "Changed",
                "Changed",
                &json!({"canvas.position":{"x":4}})
            )
            .is_err());
        assert!(store.edit_entity(&top, "", "", &json!({})).is_err());
        assert_eq!(
            before,
            serde_json::to_value(store.snapshot().unwrap()).unwrap()
        );
    }
}
