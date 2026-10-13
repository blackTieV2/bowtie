use crate::{case::CaseStore, incident::*};
use serde_json::{json, Value};
fn node(id: &str, kind: &str, diagram: &str) -> Node {
    serde_json::from_value(json!({"id":id,"kind":kind,"title":id,"barrierIds":[],"metadata":{"version":1,"diagramId":diagram,"description":"Recorded local evidence","appearance":"event","x":50.0,"y":50.0,"color":"#d8e8ee","state":if kind=="barrier" {"failed"}else{""},"barrierType":"","failureShort":"Observed failure","failureDetail":"Evidence preserved","challenge":"","response":"","causes":[],"actorId":null,"columnId":null,"location":"","category":"","confidence":"","documents":[],"caseEventIds":[],"equivalentDiagramId":null,"userData":{}}})).unwrap()
}
fn example(method: &str) -> Incident {
    serde_json::from_value(json!({"id":"investigation","title":"Transfer incident","methodKey":method,"metadata":{"version":1,"code":"I01","description":"Local investigation","whatHappened":"Liquid released during transfer","date":"2026-10-13T09:00","timeZone":"Local","incidentType":"","category":"","tier":"","unit":"Operations","diagrams":[{"id":"analysis","title":"Event analysis","timeline":false,"completed":false,"layout":"free","columns":[],"hideUnusedActors":false,"zones":if method=="categorized_rca" {vec!["Initial context","Sequence","Direct causes","Contributing causes","Root causes"]}else{vec![]}}],"taxonomy":[],"recommendations":[]},"nodes":[],"edges":[]})).unwrap()
}
fn fixture() -> (CaseStore, String) {
    let mut s = CaseStore::default();
    let root = s.create("Transfer review", "Workshop").unwrap().tree[0]
        .id
        .clone();
    let group = s
        .add_group(&root, "Station")
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "group")
        .unwrap()
        .id;
    let top = s
        .add_diagram(&group, "Liquid transfer", "Release")
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "top_event")
        .unwrap()
        .id;
    let threat = s
        .add_entity(&top, "threat", "Hose rupture", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "threat")
        .unwrap()
        .id;
    let barrier = s
        .add_entity(&threat, "barrier", "Inspection", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "barrier")
        .unwrap()
        .id;
    (s, barrier)
}
fn current(s: &CaseStore) -> Incident {
    s.snapshot().unwrap().unwrap().incidents[0].clone()
}
fn snapshot(s: &CaseStore) -> Value {
    serde_json::to_value(s.snapshot().unwrap().unwrap()).unwrap()
}
#[test]
fn documented_event_flows_barrier_paths_trios_and_rca_behaviors() {
    for method in ["systemic", "trio", "bfa", "rca", "categorized_rca"] {
        let (mut s, barrier) = fixture();
        let mut a = example(method);
        a.nodes = vec![
            node("first", "event", "analysis"),
            node("last", "event", "analysis"),
            node("parallel", "event", "analysis"),
        ];
        if ["rca", "categorized_rca"].contains(&method) {
            a.nodes[0].barrier_ids = vec![barrier];
            a.nodes[0].metadata.appearance = "root_cause".into();
            a.edges = vec![
                Edge {
                    source_id: "first".into(),
                    target_id: "last".into(),
                },
                Edge {
                    source_id: "first".into(),
                    target_id: "parallel".into(),
                },
            ];
        } else {
            let mut b = node("barrier", "barrier", "analysis");
            b.barrier_ids = vec![barrier];
            b.metadata.causes = vec![Cause {
                id: "path".into(),
                texts: vec![
                    "Local immediate finding".into(),
                    "Local basic finding".into(),
                    "Local management finding".into(),
                ],
                categories: vec![vec![], vec![], vec![]],
                inadequacy: "Recorded inadequacy".into(),
                risk_factor: "".into(),
            }];
            a.nodes.push(b);
            a.edges = vec![
                Edge {
                    source_id: "first".into(),
                    target_id: "barrier".into(),
                },
                Edge {
                    source_id: "barrier".into(),
                    target_id: "last".into(),
                },
                Edge {
                    source_id: "first".into(),
                    target_id: "parallel".into(),
                },
            ];
            if method == "trio" {
                let mut agent = node("agent", "actor", "analysis");
                agent.metadata.appearance = "agent".into();
                let mut object = node("object", "actor", "analysis");
                object.metadata.appearance = "object".into();
                a.nodes.extend([agent, object]);
                a.edges.extend([
                    Edge {
                        source_id: "agent".into(),
                        target_id: "first".into(),
                    },
                    Edge {
                        source_id: "object".into(),
                        target_id: "first".into(),
                    },
                ]);
            }
        }
        let snap = s.save_incident(&a, None).unwrap();
        assert_eq!(snap.incidents.len(), 1);
        assert_eq!(
            snap.incidents[0].edges.len(),
            if method == "trio" {
                5
            } else if ["rca", "categorized_rca"].contains(&method) {
                2
            } else {
                3
            }
        );
        assert_eq!(
            s.active
                .as_ref()
                .unwrap()
                .db
                .query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            2
        );
        let old = current(&s);
        let mut bad = old.clone();
        bad.nodes[0].metadata.appearance = "invented".into();
        if bad.nodes[0].kind == "event" {
            assert!(s.save_incident(&bad, Some(&old)).is_err());
        }
    }
}
#[test]
fn edge_node_identity_kind_ownership_and_failed_write_atomicity() {
    let (mut s, _) = fixture();
    let mut a = example("systemic");
    a.nodes = vec![
        node("event-one", "event", "analysis"),
        node("event-two", "event", "analysis"),
    ];
    a.edges = vec![Edge {
        source_id: "event-one".into(),
        target_id: "event-two".into(),
    }];
    s.save_incident(&a, None).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let file = folder.path().join("incident.sqlite");
    s.save(Some(&file)).unwrap();
    let bytes = std::fs::read(&file).unwrap();
    let old = current(&s);
    let before = snapshot(&s);
    for operation in 0..5 {
        let mut bad = old.clone();
        match operation {
            0 => bad.edges[0].target_id = "unknown".into(),
            1 => bad.edges.push(bad.edges[0].clone()),
            2 => bad.nodes[0].kind = "evidence".into(),
            3 => bad.nodes[0].metadata.version = 2,
            _ => bad.method_key = "rca".into(),
        };
        assert!(s.save_incident(&bad, Some(&old)).is_err());
        assert_eq!(snapshot(&s), before);
        assert_eq!(std::fs::read(&file).unwrap(), bytes);
    }
    let db = &s.active.as_ref().unwrap().db;
    for sql in [
        "UPDATE incident_node SET id=? WHERE id=?",
        "UPDATE incident_node SET kind=? WHERE id=?",
        "UPDATE incident_node SET incident_id=? WHERE id=?",
    ] {
        assert!(db
            .execute(sql, rusqlite::params!["changed", "event-one"])
            .is_err());
    }
    let mut other = old.clone();
    other.id = "other".into();
    assert!(s.save_incident(&other, None).is_err());
    assert_eq!(snapshot(&s), before);
}
#[test]
fn timeline_cell_moves_actors_columns_and_equivalent_diagrams() {
    let (mut s, _) = fixture();
    let mut a = example("rca");
    let mut timeline = a.metadata.diagrams[0].clone();
    timeline.id = "timeline".into();
    timeline.timeline = true;
    timeline.zones.clear();
    timeline.columns = vec![
        Column {
            id: "t1".into(),
            timestamp: "2026-10-13T09:00".into(),
            format: "date_time".into(),
            qualitative: "Preparation".into(),
            color: "#eeeeee".into(),
        },
        Column {
            id: "t2".into(),
            timestamp: "2026-10-13T10:00".into(),
            format: "qualitative".into(),
            qualitative: "First deviation".into(),
            color: "#eeeeee".into(),
        },
    ];
    a.metadata.diagrams.push(timeline);
    let actor = node("operator", "actor", "timeline");
    let mut event = node("observation", "evidence", "timeline");
    event.metadata.actor_id = Some("operator".into());
    event.metadata.column_id = Some("t1".into());
    let mut equivalent = a.metadata.diagrams[0].clone();
    equivalent.id = "detail".into();
    a.metadata.diagrams.push(equivalent);
    let mut root = node("root-event", "event", "analysis");
    root.metadata.equivalent_diagram_id = Some("detail".into());
    a.nodes = vec![actor, event, root];
    s.save_incident(&a, None).unwrap();
    let old = current(&s);
    let mut moved = old.clone();
    moved
        .nodes
        .iter_mut()
        .find(|n| n.id == "observation")
        .unwrap()
        .metadata
        .column_id = Some("t2".into());
    s.save_incident(&moved, Some(&old)).unwrap();
    let old = current(&s);
    let before = snapshot(&s);
    let mut bad = old.clone();
    bad.nodes
        .iter_mut()
        .find(|n| n.id == "observation")
        .unwrap()
        .metadata
        .actor_id = Some("root-event".into());
    assert!(s.save_incident(&bad, Some(&old)).is_err());
    assert_eq!(snapshot(&s), before);
    assert!(!timestamp("éééééééé"));
}
#[test]
fn all_incident_data_reopens_and_survives_original_csv_roundtrip() {
    let (mut s, barrier) = fixture();
    let mut a = example("rca");
    let mut event = node("event", "event", "analysis");
    event.barrier_ids = vec![barrier];
    event.metadata.category = "User category".into();
    event.metadata.user_data = json!({"record":"Local note"});
    a.nodes.push(event);
    a.metadata.recommendations.push(Recommendation {
        id: "rec".into(),
        node_id: Some("event".into()),
        diagram_id: Some("analysis".into()),
        title: "Improve inspection".into(),
        description: "Recorded recommendation".into(),
        party: "Operations".into(),
        priority: "Review".into(),
        target_date: "2026-11-01".into(),
        completed: false,
        completion_date: "".into(),
        comment: "Evidence retained".into(),
    });
    s.save_incident(&a, None).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let file = folder.path().join("case.sqlite");
    s.save(Some(&file)).unwrap();
    let old = current(&s);
    s.close();
    s.open(&file).unwrap();
    assert_eq!(current(&s), old);
    let csv = s.table_export().unwrap().text;
    let mut target = CaseStore::default();
    target.create("New", "Org").unwrap();
    let preview = target.import_preview(&csv).unwrap();
    assert!(preview.errors.is_empty(), "{:?}", preview.errors);
    target.import_apply(&csv, &preview.signature).unwrap();
    assert_eq!(current(&target), old);
    assert_eq!(target.table_export().unwrap().text, csv);
    let old = current(&target);
    target.delete_incident(&old.id, &old).unwrap();
    assert!(target.snapshot().unwrap().unwrap().incidents.is_empty());
    target.undo_tool().unwrap();
    assert_eq!(current(&target), old);
}
