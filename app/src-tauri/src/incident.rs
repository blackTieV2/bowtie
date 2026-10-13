//! Original incident services. Approved relational edges/links and versioned owned metadata.
use crate::case::CaseStore;
use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub const METHODS: &[&str] = &[
    "systemic",
    "trio",
    "bfa",
    "rca",
    "categorized_rca",
    "timeline",
];
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub method_key: String,
    pub metadata: IncidentMeta,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IncidentMeta {
    pub version: u32,
    pub code: String,
    pub description: String,
    pub what_happened: String,
    pub date: String,
    pub time_zone: String,
    pub incident_type: String,
    pub category: String,
    pub tier: String,
    pub unit: String,
    pub diagrams: Vec<Diagram>,
    pub taxonomy: Vec<Taxon>,
    pub recommendations: Vec<Recommendation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Diagram {
    pub id: String,
    pub title: String,
    pub timeline: bool,
    pub completed: bool,
    pub layout: String,
    pub columns: Vec<Column>,
    pub hide_unused_actors: bool,
    pub zones: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Column {
    pub id: String,
    pub timestamp: String,
    pub format: String,
    pub qualitative: String,
    pub color: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Taxon {
    pub id: String,
    pub level: u8,
    pub parent_id: Option<String>,
    pub code: String,
    pub title: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Recommendation {
    pub id: String,
    pub node_id: Option<String>,
    pub diagram_id: Option<String>,
    pub title: String,
    pub description: String,
    pub party: String,
    pub priority: String,
    pub target_date: String,
    pub completed: bool,
    pub completion_date: String,
    pub comment: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub metadata: NodeMeta,
    pub barrier_ids: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeMeta {
    pub version: u32,
    pub diagram_id: String,
    pub description: String,
    pub appearance: String,
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub state: String,
    pub barrier_type: String,
    pub failure_short: String,
    pub failure_detail: String,
    pub challenge: String,
    pub response: String,
    pub causes: Vec<Cause>,
    pub actor_id: Option<String>,
    pub column_id: Option<String>,
    pub location: String,
    pub category: String,
    pub confidence: String,
    pub documents: Vec<String>,
    pub case_event_ids: Vec<String>,
    pub equivalent_diagram_id: Option<String>,
    pub user_data: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cause {
    pub id: String,
    pub texts: Vec<String>,
    pub categories: Vec<Vec<String>>,
    pub inadequacy: String,
    pub risk_factor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Edge {
    pub source_id: String,
    pub target_id: String,
}
fn required(s: &str) -> Result<()> {
    if s.trim().is_empty() {
        Err("Incident identity/title is required".into())
    } else {
        Ok(())
    }
}
fn color(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}
pub fn timestamp(s: &str) -> bool {
    if !s.is_ascii() || s.len() != 16 || s.as_bytes()[10] != b'T' {
        return false;
    }
    let (date, time) = s.split_at(10);
    crate::properties::valid_date(date)
        && time.as_bytes()[3] == b':'
        && time[1..3].parse::<u8>().is_ok_and(|h| h < 24)
        && time[4..6].parse::<u8>().is_ok_and(|m| m < 60)
}
fn unique<'a>(ids: impl Iterator<Item = &'a String>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for id in ids {
        required(id)?;
        if !seen.insert(id) {
            return Err("Duplicate incident identity".into());
        }
    }
    Ok(())
}
fn valid(a: &Incident) -> Result<()> {
    required(&a.id)?;
    required(&a.title)?;
    if !METHODS.contains(&a.method_key.as_str()) || a.metadata.version != 1 {
        return Err("Unsupported incident method or metadata version".into());
    }
    if !a.metadata.date.is_empty() && !timestamp(&a.metadata.date) {
        return Err("Incident date must be YYYY-MM-DDTHH:mm".into());
    }
    unique(a.nodes.iter().map(|n| &n.id))?;
    unique(a.metadata.diagrams.iter().map(|d| &d.id))?;
    unique(a.metadata.taxonomy.iter().map(|t| &t.id))?;
    unique(a.metadata.recommendations.iter().map(|r| &r.id))?;
    for d in &a.metadata.diagrams {
        required(&d.title)?;
        if !["grid", "free"].contains(&d.layout.as_str()) {
            return Err("Invalid incident layout".into());
        }
        if !d.timeline && (a.method_key == "timeline" || !d.columns.is_empty()) {
            return Err("Invalid analysis/timeline diagram".into());
        }
        if !d.zones.is_empty()
            && (a.method_key != "categorized_rca" || d.timeline || d.zones.len() != 5)
        {
            return Err("Invalid categorized analysis zones".into());
        }
        unique(d.columns.iter().map(|c| &c.id))?;
        for c in &d.columns {
            if !timestamp(&c.timestamp)
                || !["date_time", "date", "time", "qualitative"].contains(&c.format.as_str())
                || !color(&c.color)
            {
                return Err("Invalid timeline column".into());
            }
        }
    }
    for t in &a.metadata.taxonomy {
        required(&t.title)?;
        if !(1..=3).contains(&t.level) {
            return Err("Invalid causation level".into());
        }
        if t.parent_id.as_ref().is_some_and(|p| {
            !a.metadata
                .taxonomy
                .iter()
                .any(|v| &v.id == p && v.level + 1 == t.level)
        }) {
            return Err("Taxonomy parent/level is invalid".into());
        }
    }
    for n in &a.nodes {
        required(&n.title)?;
        if !["event", "barrier", "actor", "evidence"].contains(&n.kind.as_str())
            || n.metadata.version != 1
        {
            return Err("Invalid incident node kind/version".into());
        }
        let m = &n.metadata;
        let d = a
            .metadata
            .diagrams
            .iter()
            .find(|d| d.id == m.diagram_id)
            .ok_or("Node references an unknown diagram")?;
        if !m.x.is_finite()
            || !m.y.is_finite()
            || m.x.abs() > 100000.0
            || m.y.abs() > 100000.0
            || !color(&m.color)
            || !m.user_data.is_object()
        {
            return Err("Invalid incident position/color/user data".into());
        }
        if d.timeline {
            if n.kind == "barrier"
                || !n.barrier_ids.is_empty()
                || !m.causes.is_empty()
                || !m.case_event_ids.is_empty()
            {
                return Err("Invalid timeline node data".into());
            }
            if n.kind == "actor" {
                if m.actor_id.is_some() || m.column_id.is_some() {
                    return Err("Actor row cannot occupy a cell".into());
                }
            } else {
                if !d
                    .columns
                    .iter()
                    .any(|c| Some(&c.id) == m.column_id.as_ref())
                {
                    return Err("Timeline event requires a valid column".into());
                }
                if m.actor_id.as_ref().is_some_and(|id| {
                    !a.nodes
                        .iter()
                        .any(|v| &v.id == id && v.kind == "actor" && v.metadata.diagram_id == d.id)
                }) {
                    return Err("Timeline actor is invalid".into());
                }
            }
        } else {
            if m.actor_id.is_some() || m.column_id.is_some() {
                return Err("Analysis node cannot reference timeline cells".into());
            }
            let rca = ["rca", "categorized_rca"].contains(&a.method_key.as_str());
            if rca && n.kind == "barrier" {
                return Err("Root-cause analysis represents all causes as events".into());
            }
            let appearances = if rca {
                vec![
                    "event",
                    "root_cause",
                    "contributing_cause",
                    "not_cause",
                    "more_information",
                    "stop",
                ]
            } else if a.method_key == "trio" {
                vec![
                    "event",
                    "agent",
                    "object",
                    "event_agent",
                    "event_object",
                    "potential",
                    "threat",
                    "consequence",
                    "top_event",
                ]
            } else {
                vec!["event", "potential", "threat", "consequence", "top_event"]
            };
            if n.kind == "event" && !appearances.contains(&m.appearance.as_str()) {
                return Err("Unsupported method appearance".into());
            }
            if n.kind == "actor"
                && (a.method_key != "trio" || !["agent", "object"].contains(&m.appearance.as_str()))
            {
                return Err("Analysis actor requires an agent/object trio".into());
            }
            if n.kind == "barrier"
                && !["failed", "missing", "unreliable", "inadequate", "effective"]
                    .contains(&m.state.as_str())
            {
                return Err("Invalid incident barrier state".into());
            }
            if !m.causes.is_empty() && n.kind != "barrier" {
                return Err("Causation paths belong to incident barriers".into());
            }
            unique(m.causes.iter().map(|c| &c.id))?;
            for c in &m.causes {
                if c.texts.len() != 3 || c.categories.len() != 3 {
                    return Err("Causation path requires three levels".into());
                }
                for (level, refs) in c.categories.iter().enumerate() {
                    unique(refs.iter())?;
                    for id in refs {
                        if !a
                            .metadata
                            .taxonomy
                            .iter()
                            .any(|t| &t.id == id && t.level == level as u8 + 1)
                        {
                            return Err("Unknown causation taxonomy reference".into());
                        }
                    }
                }
            }
        }
        if !n.barrier_ids.is_empty()
            && !(n.kind == "barrier"
                || n.kind == "event" && ["rca", "categorized_rca"].contains(&a.method_key.as_str()))
        {
            return Err("Invalid barrier-link source for method".into());
        }
        unique(n.barrier_ids.iter())?;
        unique(m.documents.iter())?;
        unique(m.case_event_ids.iter())?;
        if m.equivalent_diagram_id.as_ref().is_some_and(|id| {
            d.timeline
                || !["rca", "categorized_rca"].contains(&a.method_key.as_str())
                || id == &d.id
                || !a
                    .metadata
                    .diagrams
                    .iter()
                    .any(|v| &v.id == id && !v.timeline)
        }) {
            return Err("Equivalent analysis diagram is invalid".into());
        }
    }
    let mut edges = BTreeSet::new();
    for e in &a.edges {
        if e.source_id == e.target_id || !edges.insert((&e.source_id, &e.target_id)) {
            return Err("Invalid or duplicate incident edge".into());
        }
        let s = a
            .nodes
            .iter()
            .find(|n| n.id == e.source_id)
            .ok_or("Unknown connector source")?;
        let t = a
            .nodes
            .iter()
            .find(|n| n.id == e.target_id)
            .ok_or("Unknown connector target")?;
        if s.metadata.diagram_id != t.metadata.diagram_id
            || a.metadata
                .diagrams
                .iter()
                .any(|d| d.id == s.metadata.diagram_id && d.timeline)
            || s.kind == "evidence"
            || t.kind == "evidence"
        {
            return Err("Connector requires analysis nodes in the same diagram".into());
        }
    }
    for r in &a.metadata.recommendations {
        required(&r.title)?;
        if r.node_id
            .as_ref()
            .is_some_and(|id| !a.nodes.iter().any(|n| &n.id == id))
            || r.diagram_id
                .as_ref()
                .is_some_and(|id| !a.metadata.diagrams.iter().any(|d| &d.id == id))
        {
            return Err("Recommendation references an unknown element".into());
        }
        for date in [&r.target_date, &r.completion_date] {
            if !date.is_empty() && !crate::properties::valid_date(date) {
                return Err("Invalid recommendation date".into());
            }
        }
    }
    Ok(())
}
pub(crate) fn incidents(db: &Connection) -> Result<Vec<Incident>> {
    let mut q = db
        .prepare("SELECT id,title,method_key,metadata_json FROM incident ORDER BY title,id")
        .map_err(err)?;
    let base = q
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })
        .map_err(err)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(err)?;
    let mut out = Vec::new();
    for (id, title, method_key, meta) in base {
        let mut q=db.prepare("SELECT id,kind,title,metadata_json FROM incident_node WHERE incident_id=? ORDER BY id").map_err(err)?;
        let raw = q
            .query_map([&id], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                ))
            })
            .map_err(err)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(err)?;
        let mut nodes = Vec::new();
        for (nid, kind, ntitle, metadata) in raw {
            let mut q=db.prepare("SELECT barrier_id FROM incident_barrier_link WHERE incident_node_id=? ORDER BY barrier_id").map_err(err)?;
            let barrier_ids = q
                .query_map([&nid], |r| r.get(0))
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<String>>>()
                .map_err(err)?;
            nodes.push(Node {
                id: nid,
                kind,
                title: ntitle,
                metadata: serde_json::from_str(&metadata).map_err(err)?,
                barrier_ids,
            });
        }
        let mut q=db.prepare("SELECT source_id,target_id FROM incident_edge WHERE incident_id=? ORDER BY source_id,target_id").map_err(err)?;
        let edges = q
            .query_map([&id], |r| {
                Ok(Edge {
                    source_id: r.get(0)?,
                    target_id: r.get(1)?,
                })
            })
            .map_err(err)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(err)?;
        out.push(Incident {
            id,
            title,
            method_key,
            metadata: serde_json::from_str(&meta).map_err(err)?,
            nodes,
            edges,
        });
    }
    Ok(out)
}
pub(crate) fn validate(db: &Connection) -> Result<()> {
    for a in incidents(db)? {
        valid(&a)?;
        for n in &a.nodes {
            for (ids, kinds) in [
                (&n.barrier_ids, vec!["barrier"]),
                (&n.metadata.documents, vec!["document"]),
                (
                    &n.metadata.case_event_ids,
                    vec![
                        "hazard",
                        "threat",
                        "top_event",
                        "consequence",
                        "escalation_factor",
                    ],
                ),
            ] {
                for id in ids {
                    let kind: String = db
                        .query_row("SELECT kind FROM entity WHERE id=?", [id], |r| r.get(0))
                        .map_err(err)?;
                    if !kinds.contains(&kind.as_str()) {
                        return Err("Invalid incident reference target kind".into());
                    }
                }
            }
        }
    }
    Ok(())
}
pub(crate) fn write_bundle(tx: &Transaction<'_>, a: &Incident) -> Result<()> {
    valid(a)?;
    let old = incidents(tx)?.into_iter().find(|v| v.id == a.id);
    if old
        .as_ref()
        .is_some_and(|v| !v.nodes.is_empty() && v.method_key != a.method_key)
    {
        return Err("Incident analysis method cannot change after nodes exist".into());
    }
    let version: i64 = tx
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(err)?;
    if version == 1 {
        tx.execute_batch(crate::RCA_LINK_MIGRATION).map_err(err)?;
    }
    tx.execute("INSERT INTO incident(id,title,method_key,metadata_json) VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET title=excluded.title,method_key=excluded.method_key,metadata_json=excluded.metadata_json",params![a.id,a.title,a.method_key,serde_json::to_string(&a.metadata).map_err(err)?]).map_err(err)?;
    tx.execute("DELETE FROM incident_edge WHERE incident_id=?", [&a.id])
        .map_err(err)?;
    tx.execute("DELETE FROM incident_barrier_link WHERE incident_node_id IN (SELECT id FROM incident_node WHERE incident_id=?)",[&a.id]).map_err(err)?;
    for n in old.iter().flat_map(|v| &v.nodes) {
        if !a.nodes.iter().any(|v| v.id == n.id) {
            tx.execute("DELETE FROM incident_node WHERE id=?", [&n.id])
                .map_err(err)?;
        }
    }
    for n in &a.nodes {
        let stored = tx.query_row(
            "SELECT incident_id,kind FROM incident_node WHERE id=?",
            [&n.id],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        );
        match stored {
            Ok((owner, kind)) if owner != a.id || kind != n.kind => {
                return Err("Incident node identity/kind/owner are immutable".into())
            }
            Err(rusqlite::Error::QueryReturnedNoRows) | Ok(_) => {}
            Err(e) => return Err(err(e)),
        }
        tx.execute("INSERT INTO incident_node(id,incident_id,kind,title,metadata_json) VALUES(?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET title=excluded.title,metadata_json=excluded.metadata_json",params![n.id,a.id,n.kind,n.title,serde_json::to_string(&n.metadata).map_err(err)?]).map_err(err)?;
        for id in &n.barrier_ids {
            tx.execute(
                "INSERT INTO incident_barrier_link(incident_node_id,barrier_id) VALUES(?,?)",
                params![n.id, id],
            )
            .map_err(err)?;
        }
    }
    for e in &a.edges {
        tx.execute(
            "INSERT INTO incident_edge(incident_id,source_id,target_id) VALUES(?,?,?)",
            params![a.id, e.source_id, e.target_id],
        )
        .map_err(err)?;
    }
    validate(tx)
}
pub(crate) fn references(db: &Connection, id: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for a in incidents(db)? {
        for n in a.nodes {
            if n.metadata
                .documents
                .iter()
                .chain(&n.metadata.case_event_ids)
                .any(|v| v == id)
            {
                out.push(format!("incident {} / {}", a.title, n.title));
            }
        }
    }
    Ok(out)
}
impl CaseStore {
    pub fn save_incident(
        &mut self,
        a: &Incident,
        expected: Option<&Incident>,
    ) -> Result<crate::case::CaseSnapshot> {
        let checkpoint = self.undo_checkpoint()?;
        let c = self.active.as_mut().ok_or("No case is open")?;
        let old = incidents(&c.db)?.into_iter().find(|v| v.id == a.id);
        if old.as_ref() != expected {
            return Err("Incident changed; reopen its editor".into());
        }
        let tx = c.db.transaction().map_err(err)?;
        write_bundle(&tx, a)?;
        crate::case::validate(&tx)?;
        tx.commit().map_err(err)?;
        c.dirty = true;
        self.remember_tool_undo(checkpoint)?;
        self.snapshot()?.ok_or("No case is open".into())
    }
    pub fn delete_incident(
        &mut self,
        id: &str,
        expected: &Incident,
    ) -> Result<crate::case::CaseSnapshot> {
        let checkpoint = self.undo_checkpoint()?;
        let c = self.active.as_mut().ok_or("No case is open")?;
        let old = incidents(&c.db)?
            .into_iter()
            .find(|v| v.id == id)
            .ok_or("Incident not found")?;
        if &old != expected {
            return Err("Incident changed; reopen its editor".into());
        }
        let tx = c.db.transaction().map_err(err)?;
        tx.execute("DELETE FROM incident_edge WHERE incident_id=?", [id])
            .map_err(err)?;
        tx.execute("DELETE FROM incident_barrier_link WHERE incident_node_id IN (SELECT id FROM incident_node WHERE incident_id=?)",[id]).map_err(err)?;
        tx.execute("DELETE FROM incident_node WHERE incident_id=?", [id])
            .map_err(err)?;
        tx.execute("DELETE FROM incident WHERE id=?", [id])
            .map_err(err)?;
        tx.commit().map_err(err)?;
        c.dirty = true;
        self.remember_tool_undo(checkpoint)?;
        self.snapshot()?.ok_or("No case is open".into())
    }
}
