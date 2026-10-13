use crate::{
    case::{validate, CaseStore},
    INITIAL_SCHEMA, RCA_LINK_MIGRATION,
};
use rusqlite::{params, Connection};
#[test]
fn real_v1_file_migrates_in_working_copy_and_roundtrips() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("v1.sqlite");
    {
        let db = Connection::open(&path).unwrap();
        db.execute_batch(INITIAL_SCHEMA).unwrap();
        db.execute(
            "INSERT INTO case_info(id,title,metadata_json) VALUES(1,?,?)",
            params![
                "Old case",
                r#"{"application":"barrier-atlas","format_version":1}"#
            ],
        )
        .unwrap();
        db.execute(
            "INSERT INTO entity(id,kind,title) VALUES(?,?,?)",
            params!["root", "organization", "Organization"],
        )
        .unwrap();
        validate(&db).unwrap();
    }
    let bytes = std::fs::read(&path).unwrap();
    let mut store = CaseStore::default();
    let opened = store.open(&path).unwrap();
    assert!(!opened.dirty);
    assert_eq!(
        store
            .active
            .as_ref()
            .unwrap()
            .db
            .query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        2
    );
    assert_eq!(bytes, std::fs::read(&path).unwrap());
    store.save(None).unwrap();
    store.close();
    let reopened = store.open(&path).unwrap();
    assert!(!reopened.dirty);
    assert_eq!(opened.title, reopened.title);
    assert_eq!(
        serde_json::to_value(opened.tree).unwrap(),
        serde_json::to_value(reopened.tree).unwrap()
    );
    let db = Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        2
    );
    validate(&db).unwrap();
}
#[test]
fn method_conditional_links_insert_update_and_unchanged_identity() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(INITIAL_SCHEMA).unwrap();
    let before = objects(&db);
    db.execute_batch(RCA_LINK_MIGRATION).unwrap();
    let after = objects(&db);
    for (kind, name, sql) in &before {
        if name != "incident_link_insert" && name != "incident_link_update" {
            assert!(after.contains(&(kind.clone(), name.clone(), sql.clone())));
        }
    }
    assert_eq!(before.len(), after.len());
    db.execute(
        "INSERT INTO entity(id,kind,title) VALUES(?,?,?)",
        params!["root", "organization", "Organization"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["group", "root", "group", "Group"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["hazard", "group", "hazard", "Hazard"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["diagram", "hazard", "diagram", "Diagram"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["top", "diagram", "top_event", "Top"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["threat", "top", "threat", "Threat"],
    )
    .unwrap();
    db.execute(
        "INSERT INTO entity(id,parent_id,kind,title) VALUES(?,?,?,?)",
        params!["case-barrier", "threat", "barrier", "Barrier"],
    )
    .unwrap();
    for method in [
        "systemic",
        "tripod",
        "bfa",
        "rca",
        "categorized_rca",
        "timeline",
        "basic",
        "BSCAT",
        "unknown",
    ] {
        db.execute(
            "INSERT INTO incident(id,title,method_key) VALUES(?,?,?)",
            params![method, method, method],
        )
        .unwrap();
        for kind in ["barrier", "event", "actor", "evidence"] {
            let id = format!("{method}-{kind}");
            db.execute(
                "INSERT INTO incident_node(id,incident_id,kind,title) VALUES(?,?,?,?)",
                params![id, method, kind, kind],
            )
            .unwrap();
            let allowed = kind == "barrier"
                || kind == "event" && ["rca", "categorized_rca"].contains(&method);
            assert_eq!(
                db.execute(
                    "INSERT INTO incident_barrier_link(incident_node_id,barrier_id) VALUES(?,?)",
                    params![id, "case-barrier"]
                )
                .is_ok(),
                allowed,
                "{method}/{kind}"
            );
            if allowed {
                assert!(db
                    .execute(
                        "UPDATE incident_barrier_link SET barrier_id=? WHERE incident_node_id=?",
                        params!["threat", id]
                    )
                    .is_err());
                let actor = format!("{method}-{kind}-reject-actor");
                db.execute(
                    "INSERT INTO incident_node(id,incident_id,kind,title) VALUES(?,?,?,?)",
                    params![actor, method, "actor", "Actor"],
                )
                .unwrap();
                assert!(db.execute("UPDATE incident_barrier_link SET incident_node_id=? WHERE incident_node_id=?",params![actor,id]).is_err());
            }
            assert!(db
                .execute(
                    "UPDATE incident_node SET kind=? WHERE id=?",
                    params![
                        if kind == "barrier" {
                            "event"
                        } else {
                            "barrier"
                        },
                        id
                    ]
                )
                .is_err());
        }
    }
}
fn objects(db: &Connection) -> Vec<(String, String, String)> {
    db.prepare("SELECT type,name,sql FROM sqlite_schema WHERE sql IS NOT NULL AND name NOT LIKE 'sqlite_%' ORDER BY type,name").unwrap().query_map([],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).unwrap().map(Result::unwrap).collect()
}
