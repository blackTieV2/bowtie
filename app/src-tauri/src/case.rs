//! Original local-case service. No raw SQL or generic filesystem IPC is exposed.
use rusqlite::{
    backup::{Backup, StepResult},
    params, Connection, OpenFlags,
};
use serde::Serialize;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};
use tempfile::NamedTempFile;
use uuid::Uuid;

type CaseResult<T> = Result<T, String>;
fn error_text(error: impl Display) -> String {
    error.to_string()
}
fn new_id() -> String {
    Uuid::new_v4().to_string()
}

// Every connection, including readers, scratch validators and save destinations,
// passes through this function. Foreign keys are a connection setting.
fn configured(connection: Connection) -> CaseResult<Connection> {
    connection
        .pragma_update(None, "foreign_keys", true)
        .map_err(error_text)?;
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(error_text)?;
    let enabled: i64 = connection
        .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
        .map_err(error_text)?;
    if enabled != 1 {
        return Err("Foreign keys could not be enabled".into());
    }
    Ok(connection)
}
pub(crate) fn memory() -> CaseResult<Connection> {
    configured(Connection::open_in_memory().map_err(error_text)?)
}
pub(crate) fn read_only(path: &Path) -> CaseResult<Connection> {
    configured(
        Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(error_text)?,
    )
}
pub(crate) fn copy_database(from: &Connection, to: &mut Connection) -> CaseResult<()> {
    let backup = Backup::new(from, to).map_err(error_text)?;
    match backup.step(-1).map_err(error_text)? {
        StepResult::Done => Ok(()),
        _ => Err("The case file is busy; please retry".into()),
    }
}
fn schema_objects(db: &Connection) -> CaseResult<Vec<(String, String, String)>> {
    let mut query = db.prepare("SELECT type,name,sql FROM sqlite_schema WHERE sql IS NOT NULL AND name NOT LIKE 'sqlite_%' ORDER BY type,name").map_err(error_text)?;
    let rows = query
        .query_map([], |r| {
            let sql: String = r.get(2)?;
            Ok((r.get(0)?, r.get(1)?, sql.replace("\r\n", "\n")))
        })
        .map_err(error_text)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(error_text)
}
pub(crate) fn validate(db: &Connection) -> CaseResult<()> {
    let version: i64 = db
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(error_text)?;
    if !matches!(version, 1 | 2) {
        return Err("Unsupported Barrier Atlas case version".into());
    }
    let expected = memory()?;
    expected
        .execute_batch(crate::INITIAL_SCHEMA)
        .map_err(error_text)?;
    if version == 2 {
        expected
            .execute_batch(crate::RCA_LINK_MIGRATION)
            .map_err(error_text)?;
    }
    if schema_objects(db)? != schema_objects(&expected)? {
        return Err("This file does not have the supported Barrier Atlas case schema".into());
    }
    let format: String = db
        .query_row(
            "SELECT json_extract(metadata_json,'$.application') FROM case_info WHERE id=1",
            [],
            |r| r.get(0),
        )
        .map_err(error_text)?;
    if format != "barrier-atlas" {
        return Err("This file is not a Barrier Atlas case".into());
    }
    let integrity: String = db
        .query_row("PRAGMA quick_check", [], |r| r.get(0))
        .map_err(error_text)?;
    if integrity != "ok" {
        return Err("Case integrity check failed".into());
    }
    if db
        .prepare("PRAGMA foreign_key_check")
        .map_err(error_text)?
        .query([])
        .map_err(error_text)?
        .next()
        .map_err(error_text)?
        .is_some()
    {
        return Err("Case references are invalid".into());
    }
    let roots: i64 = db
        .query_row(
            "SELECT count(*) FROM entity WHERE kind='organization'",
            [],
            |r| r.get(0),
        )
        .map_err(error_text)?;
    if roots != 1 {
        return Err("The case must contain one organization".into());
    }
    let unreachable: i64 = db
        .query_row(
            "SELECT (SELECT count(*) FROM entity)-(SELECT count(*) FROM case_tree)",
            [],
            |r| r.get(0),
        )
        .map_err(error_text)?;
    let incomplete: i64 = db.query_row(
        "SELECT count(*) FROM entity d WHERE d.kind='diagram' AND (SELECT count(*) FROM entity t WHERE t.kind='top_event' AND t.parent_id=d.id)<>1",
        [], |r| r.get(0),
    ).map_err(error_text)?;
    if unreachable != 0 || incomplete != 0 {
        return Err("The case has an incomplete or disconnected hierarchy".into());
    }
    crate::audit::validate(db)?;
    crate::incident::validate(db)?;
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeRow {
    pub id: String,
    pub parent_id: Option<String>,
    pub kind: String,
    pub title: String,
    pub sort_order: i64,
    pub depth: i64,
    pub description: String,
    pub properties: serde_json::Value,
    pub effectiveness_id: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkRow {
    pub source_id: String,
    pub target_id: String,
    pub relation: String,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseSnapshot {
    pub title: String,
    pub path: Option<String>,
    pub dirty: bool,
    pub tree: Vec<TreeRow>,
    pub links: Vec<LinkRow>,
    pub matrices: Vec<crate::risk::Matrix>,
    pub assessments: Vec<crate::risk::Assessment>,
    pub lookups: Vec<crate::risk::Lookup>,
    pub actions: Vec<crate::risk::ActionRow>,
    pub custom_fields: serde_json::Value,
    pub audits: Vec<crate::audit::Audit>,
    pub audit_results: std::collections::BTreeMap<String, Vec<crate::audit::ResultRow>>,
    pub incidents: Vec<crate::incident::Incident>,
}
pub(crate) struct ActiveCase {
    pub(crate) db: Connection,
    path: Option<PathBuf>,
    pub(crate) dirty: bool,
}
#[derive(Default)]
pub struct CaseStore {
    pub(crate) active: Option<ActiveCase>,
    pub(crate) tool_undo: Option<crate::case_tools::ToolUndo>,
}

impl CaseStore {
    pub fn snapshot(&self) -> CaseResult<Option<CaseSnapshot>> {
        let Some(case) = &self.active else {
            return Ok(None);
        };
        let title = case
            .db
            .query_row("SELECT title FROM case_info WHERE id=1", [], |r| r.get(0))
            .map_err(error_text)?;
        let mut query = case.db.prepare("SELECT t.id,t.parent_id,t.kind,t.title,t.sort_order,t.depth,e.description,coalesce((SELECT json_group_object(property_key,json(value_json)) FROM entity_property WHERE entity_id=t.id),'{}'),e.effectiveness_id FROM case_tree t JOIN entity e ON e.id=t.id ORDER BY t.depth,t.sort_order,t.id").map_err(error_text)?;
        let tree = query
            .query_map([], |r| {
                Ok(TreeRow {
                    id: r.get(0)?,
                    parent_id: r.get(1)?,
                    kind: r.get(2)?,
                    title: r.get(3)?,
                    sort_order: r.get(4)?,
                    depth: r.get(5)?,
                    description: r.get(6)?,
                    properties: serde_json::from_str(&r.get::<_, String>(7)?).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?,
                    effectiveness_id: r.get(8)?,
                })
            })
            .map_err(error_text)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(error_text)?;
        let mut link_query = case.db.prepare("SELECT source_id,target_id,relation FROM entity_link ORDER BY source_id,relation,target_id").map_err(error_text)?;
        let links = link_query
            .query_map([], |r| {
                Ok(LinkRow {
                    source_id: r.get(0)?,
                    target_id: r.get(1)?,
                    relation: r.get(2)?,
                })
            })
            .map_err(error_text)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(error_text)?;
        Ok(Some(CaseSnapshot {
            title,
            path: case.path.as_ref().map(|p| p.display().to_string()),
            dirty: case.dirty,
            tree,
            links,
            matrices: crate::risk::matrices(&case.db)?,
            assessments: crate::risk::assessments(&case.db)?,
            lookups: crate::risk::lookups(&case.db)?,
            actions: crate::risk::action_register(&case.db)?,
            custom_fields: crate::properties::definitions(&case.db)?,
            audits: crate::audit::audits(&case.db)?,
            audit_results: crate::audit::summaries(&case.db)?,
            incidents: crate::incident::incidents(&case.db)?,
        }))
    }
    pub fn create(&mut self, title: &str, organization: &str) -> CaseResult<CaseSnapshot> {
        let mut db = memory()?;
        db.execute_batch(crate::INITIAL_SCHEMA)
            .map_err(error_text)?;
        let tx = db.transaction().map_err(error_text)?;
        tx.execute(
            "INSERT INTO case_info(id,title,metadata_json) VALUES(1,?,?)",
            params![
                title.trim(),
                r#"{"application":"barrier-atlas","format_version":1}"#
            ],
        )
        .map_err(error_text)?;
        tx.execute(
            "INSERT INTO entity(id,kind,title) VALUES(?,'organization',?)",
            params![new_id(), organization.trim()],
        )
        .map_err(error_text)?;
        tx.commit().map_err(error_text)?;
        self.active = Some(ActiveCase {
            db,
            path: None,
            dirty: true,
        });
        self.tool_undo = None;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
    pub fn open(&mut self, path: &Path) -> CaseResult<CaseSnapshot> {
        let source = read_only(path)?;
        validate(&source)?;
        let mut db = memory()?;
        copy_database(&source, &mut db)?;
        let version: i64 = db
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(error_text)?;
        if version == 1 {
            let tx = db.transaction().map_err(error_text)?;
            tx.execute_batch(crate::RCA_LINK_MIGRATION)
                .map_err(error_text)?;
            validate(&tx)?;
            tx.commit().map_err(error_text)?;
        }
        validate(&db)?;
        self.active = Some(ActiveCase {
            db,
            path: Some(path.to_path_buf()),
            dirty: false,
        });
        self.tool_undo = None;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
    pub fn save(&mut self, destination: Option<&Path>) -> CaseResult<CaseSnapshot> {
        let case = self.active.as_mut().ok_or("No case is open")?;
        let path = destination
            .map(Path::to_path_buf)
            .or_else(|| case.path.clone())
            .ok_or("Choose a location for the case")?;
        if !path.is_absolute()
            || !path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("sqlite"))
        {
            return Err("Choose an absolute file path ending in .sqlite".into());
        }
        // Protect unrelated SQLite files even if an IPC caller supplies a path.
        if path.exists() {
            validate(&read_only(&path)?)?;
        }
        let parent = path.parent().ok_or("Choose a valid directory")?;
        let temporary = NamedTempFile::new_in(parent).map_err(error_text)?;
        {
            let mut target = configured(Connection::open(temporary.path()).map_err(error_text)?)?;
            target
                .pragma_update(None, "synchronous", "FULL")
                .map_err(error_text)?;
            copy_database(&case.db, &mut target)?;
            validate(&target)?;
        }
        temporary.as_file().sync_all().map_err(error_text)?;
        temporary.persist(&path).map_err(error_text)?;
        case.path = Some(path);
        case.dirty = false;
        self.tool_undo = None;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
    pub fn close(&mut self) {
        self.active = None;
        self.tool_undo = None;
    }
    pub fn add_group(&mut self, parent: &str, title: &str) -> CaseResult<CaseSnapshot> {
        let case = self.active.as_mut().ok_or("No case is open")?;
        let tx = case.db.transaction().map_err(error_text)?;
        let order: i64 = tx
            .query_row(
                "SELECT coalesce(max(sort_order),-1)+1 FROM entity WHERE parent_id=?",
                [parent],
                |r| r.get(0),
            )
            .map_err(error_text)?;
        tx.execute(
            "INSERT INTO entity(id,kind,parent_id,title,sort_order) VALUES(?,'group',?,?,?)",
            params![new_id(), parent, title.trim(), order],
        )
        .map_err(error_text)?;
        tx.commit().map_err(error_text)?;
        case.dirty = true;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
    pub fn add_diagram(
        &mut self,
        parent: &str,
        hazard_title: &str,
        top_event_title: &str,
    ) -> CaseResult<CaseSnapshot> {
        let case = self.active.as_mut().ok_or("No case is open")?;
        let tx = case.db.transaction().map_err(error_text)?;
        let kind: String = tx
            .query_row("SELECT kind FROM entity WHERE id=?", [parent], |r| r.get(0))
            .map_err(error_text)?;
        let hazard = match kind.as_str() {
            "hazard" => parent.to_owned(),
            "group" => {
                let id = new_id();
                tx.execute(
                    "INSERT INTO entity(id,kind,parent_id,title) VALUES(?,'hazard',?,?)",
                    params![id, parent, hazard_title.trim()],
                )
                .map_err(error_text)?;
                id
            }
            _ => return Err("Select a group or hazard for the new diagram".into()),
        };
        let diagram = new_id();
        let order: i64 = tx
            .query_row(
                "SELECT coalesce(max(sort_order),-1)+1 FROM entity WHERE parent_id=?",
                [&hazard],
                |r| r.get(0),
            )
            .map_err(error_text)?;
        tx.execute(
            "INSERT INTO entity(id,kind,parent_id,title,sort_order) VALUES(?,'diagram',?,?,?)",
            params![diagram, hazard, top_event_title.trim(), order],
        )
        .map_err(error_text)?;
        tx.execute(
            "INSERT INTO entity(id,kind,parent_id,title) VALUES(?,'top_event',?,?)",
            params![new_id(), diagram, top_event_title.trim()],
        )
        .map_err(error_text)?;
        tx.commit().map_err(error_text)?;
        case.dirty = true;
        self.snapshot()?.ok_or_else(|| "No case is open".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example() -> (CaseStore, String) {
        let mut store = CaseStore::default();
        let case = store.create("Transfer review", "Workshop").unwrap();
        let root = case.tree[0].id.clone();
        (store, root)
    }
    #[test]
    fn runtime_round_trip_and_shared_hazard() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("review.sqlite");
        let (mut store, root) = example();
        let group = store
            .add_group(&root, "Transfer station")
            .unwrap()
            .tree
            .into_iter()
            .find(|r| r.kind == "group")
            .unwrap()
            .id;
        let first = store
            .add_diagram(&group, "Liquid transfer", "Loss of containment")
            .unwrap();
        let hazard = first
            .tree
            .iter()
            .find(|r| r.kind == "hazard")
            .unwrap()
            .id
            .clone();
        let case = store
            .add_diagram(&hazard, "", "Uncontrolled release")
            .unwrap();
        assert_eq!(case.tree.len(), 7);
        assert_eq!(case.tree.iter().filter(|r| r.kind == "hazard").count(), 1);
        let mut ids = std::collections::HashSet::new();
        for node in &case.tree {
            assert_eq!(Uuid::parse_str(&node.id).unwrap().get_version_num(), 4);
            assert!(ids.insert(node.id.clone()));
        }
        let expected = serde_json::to_value(&case.tree).unwrap();
        let saved = store.save(Some(&file)).unwrap();
        assert!(!saved.dirty);
        store.close();
        let opened = store.open(&file).unwrap();
        assert!(!opened.dirty);
        assert_eq!(serde_json::to_value(&opened.tree).unwrap(), expected);
        assert_eq!(opened.title, "Transfer review");
        let root = opened
            .tree
            .iter()
            .find(|r| r.kind == "organization")
            .unwrap()
            .id
            .clone();
        store.add_group(&root, "Second location").unwrap();
        store.save(None).unwrap();
        let mut another = CaseStore::default();
        assert_eq!(another.open(&file).unwrap().tree.len(), 8);
    }
    #[test]
    fn foreign_keys_are_on_for_all_connection_types() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("case.sqlite");
        let (mut store, _) = example();
        store.save(Some(&path)).unwrap();
        let scratch = memory().unwrap();
        let reader = read_only(&path).unwrap();
        let destination =
            configured(Connection::open(folder.path().join("destination.sqlite")).unwrap())
                .unwrap();
        for db in [
            &scratch,
            &reader,
            &destination,
            &store.active.as_ref().unwrap().db,
        ] {
            assert_eq!(
                db.query_row("PRAGMA foreign_keys", [], |r| r.get::<_, i64>(0))
                    .unwrap(),
                1
            );
        }
    }
    #[test]
    fn diagram_and_top_event_roll_back_together() {
        let (mut store, root) = example();
        let group = store
            .add_group(&root, "Group")
            .unwrap()
            .tree
            .into_iter()
            .find(|r| r.kind == "group")
            .unwrap()
            .id;
        let before = store.snapshot().unwrap().unwrap().tree.len();
        store.active.as_ref().unwrap().db.execute_batch("CREATE TRIGGER reject_top BEFORE INSERT ON entity WHEN NEW.kind='top_event' BEGIN SELECT RAISE(ABORT,'Injected failure'); END;").unwrap();
        assert!(store.add_diagram(&group, "Hazard", "Top event").is_err());
        assert_eq!(store.snapshot().unwrap().unwrap().tree.len(), before);
        store
            .active
            .as_ref()
            .unwrap()
            .db
            .execute_batch("DROP TRIGGER reject_top")
            .unwrap();
        assert!(store.add_diagram(&group, "Hazard", "Top event").is_ok());
    }
    #[test]
    fn failed_open_and_create_preserve_session() {
        let folder = tempfile::tempdir().unwrap();
        let bad = folder.path().join("bad.sqlite");
        std::fs::write(&bad, b"not a sqlite case").unwrap();
        let (mut store, _) = example();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        assert!(store.open(&bad).is_err());
        assert!(store.create("", "Organization").is_err());
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            before
        );
    }
    #[test]
    fn failed_save_preserves_file_and_dirty_state() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("review.sqlite");
        let (mut store, root) = example();
        store.save(Some(&file)).unwrap();
        let original = std::fs::read(&file).unwrap();
        store.add_group(&root, "Unsaved group").unwrap();
        let bad = folder.path().join("missing").join("case.sqlite");
        assert!(store.save(Some(&bad)).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), original);
        assert!(store.snapshot().unwrap().unwrap().dirty);
        let foreign_path = folder.path().join("foreign.sqlite");
        let foreign = configured(Connection::open(&foreign_path).unwrap()).unwrap();
        foreign
            .execute_batch("CREATE TABLE unrelated(value TEXT)")
            .unwrap();
        drop(foreign);
        let bytes = std::fs::read(&foreign_path).unwrap();
        assert!(store.save(Some(&foreign_path)).is_err());
        assert_eq!(std::fs::read(&foreign_path).unwrap(), bytes);
        let copy = folder.path().join("copy.sqlite");
        assert!(!store.save(Some(&copy)).unwrap().dirty);
        assert_eq!(std::fs::read(&file).unwrap(), original);
    }
    #[test]
    fn incompatible_schema_and_version_are_rejected() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("case.sqlite");
        let (mut store, _) = example();
        store.save(Some(&file)).unwrap();
        let db = configured(Connection::open(&file).unwrap()).unwrap();
        db.pragma_update(None, "user_version", 2).unwrap();
        drop(db);
        assert!(store.open(&file).is_err());
        let db = configured(Connection::open(&file).unwrap()).unwrap();
        db.pragma_update(None, "user_version", 1).unwrap();
        db.execute_batch("DROP TRIGGER matrix_limit").unwrap();
        drop(db);
        assert!(store.open(&file).is_err());
    }
    #[test]
    fn incomplete_persisted_diagram_is_rejected() {
        let folder = tempfile::tempdir().unwrap();
        let file = folder.path().join("case.sqlite");
        let (mut store, root) = example();
        let group = store
            .add_group(&root, "Group")
            .unwrap()
            .tree
            .into_iter()
            .find(|r| r.kind == "group")
            .unwrap()
            .id;
        store.save(Some(&file)).unwrap();
        let before = serde_json::to_value(store.snapshot().unwrap()).unwrap();
        let db = configured(Connection::open(&file).unwrap()).unwrap();
        let hazard = new_id();
        db.execute(
            "INSERT INTO entity(id,kind,parent_id,title) VALUES(?,'hazard',?,'Hazard')",
            params![hazard, group],
        )
        .unwrap();
        db.execute(
            "INSERT INTO entity(id,kind,parent_id,title) VALUES(?,'diagram',?,'Incomplete')",
            params![new_id(), hazard],
        )
        .unwrap();
        drop(db);
        assert!(store.open(&file).is_err());
        assert_eq!(
            serde_json::to_value(store.snapshot().unwrap()).unwrap(),
            before
        );
    }
    #[test]
    fn schema_signature_ignores_platform_line_endings() {
        let lf = memory().unwrap();
        let crlf = memory().unwrap();
        let schema = crate::INITIAL_SCHEMA.replace("\r\n", "\n");
        lf.execute_batch(&schema).unwrap();
        crlf.execute_batch(&schema.replace('\n', "\r\n")).unwrap();
        assert_eq!(schema_objects(&lf).unwrap(), schema_objects(&crlf).unwrap());
    }
}
