use crate::{case::CaseStore, risk::*};
use serde_json::json;

#[test]
fn duplicate_category_identity_and_invalid_metadata_are_rejected() {
    let (mut store, _, _, _) = fixture();
    let mut m = matrix();
    m.categories[1].id = m.categories[0].id.clone();
    assert!(store.save_matrix(&m).is_err());
    assert!(store.snapshot().unwrap().unwrap().matrices.is_empty());
    let l = Lookup {
        id: uuid::Uuid::new_v4().to_string(),
        table_key: "effectiveness".into(),
        code: String::new(),
        title: "Rating".into(),
        color: "#aabbcc".into(),
        sort_order: 0,
        metadata: json!({"frequency":"wrong"}),
    };
    assert!(store.save_lookup(&l).is_err());
    assert!(store.snapshot().unwrap().unwrap().lookups.is_empty());
}
#[test]
fn effectiveness_custom_metadata_definitions_apply_to_all_values() {
    let (mut store, _, _, _) = fixture();
    store
        .custom_field("lookup:effectiveness", "MTBF", "number", None, false)
        .unwrap();
    let l = Lookup {
        id: uuid::Uuid::new_v4().to_string(),
        table_key: "effectiveness".into(),
        code: String::new(),
        title: "Rating".into(),
        color: "#aabbcc".into(),
        sort_order: 0,
        metadata: json!({"user_data":{"MTBF":25.5}}),
    };
    store.save_lookup(&l).unwrap();
    store
        .custom_field(
            "lookup:effectiveness",
            "Hours",
            "number",
            Some("MTBF"),
            false,
        )
        .unwrap();
    assert_eq!(
        store.snapshot().unwrap().unwrap().lookups[0].metadata["user_data"]["Hours"],
        25.5
    );
    store
        .custom_field(
            "lookup:effectiveness",
            "Hours",
            "number",
            Some("Hours"),
            true,
        )
        .unwrap();
    assert!(
        store.snapshot().unwrap().unwrap().lookups[0].metadata["user_data"]
            .as_object()
            .unwrap()
            .is_empty()
    );
}
fn fixture() -> (CaseStore, String, String, String) {
    let mut s = CaseStore::default();
    let root = s.create("Risk review", "Workshop").unwrap().tree[0]
        .id
        .clone();
    let group = s
        .add_group(&root, "Transfer")
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "group")
        .unwrap()
        .id;
    let c = s
        .add_diagram(&group, "Liquid transfer", "Containment lost")
        .unwrap();
    let top = c
        .tree
        .iter()
        .find(|n| n.kind == "top_event")
        .unwrap()
        .id
        .clone();
    let threat = s
        .add_entity(&top, "threat", "Hose rupture", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "threat")
        .unwrap()
        .id;
    (s, root, top, threat)
}
fn matrix() -> Matrix {
    let id = uuid::Uuid::new_v4().to_string();
    let low = uuid::Uuid::new_v4().to_string();
    let high = uuid::Uuid::new_v4().to_string();
    Matrix {
        id,
        title: "Safety".into(),
        coordinate_order: "xy".into(),
        likelihood: vec![
            Axis {
                code: "L".into(),
                title: "Rare".into(),
            },
            Axis {
                code: "H".into(),
                title: "Frequent".into(),
            },
        ],
        severity: vec![
            Axis {
                code: "1".into(),
                title: "Minor".into(),
            },
            Axis {
                code: "2".into(),
                title: "Major".into(),
            },
        ],
        categories: vec![
            Category {
                id: low.clone(),
                title: "Low".into(),
                color: "#bbddbb".into(),
                rank: 0,
            },
            Category {
                id: high.clone(),
                title: "High".into(),
                color: "#ddaaaa".into(),
                rank: 1,
            },
        ],
        cells: (0..2)
            .flat_map(|s| {
                (0..2).map({
                    let low = low.clone();
                    let high = high.clone();
                    move |l| Cell {
                        likelihood_position: l,
                        severity_position: s,
                        category_id: if l + s < 2 { low.clone() } else { high.clone() },
                        label: String::new(),
                    }
                })
            })
            .collect(),
    }
}
#[test]
fn maximum_eight_matrices_is_enforced_by_trigger() {
    let (mut s, _, _, _) = fixture();
    for _ in 0..8 {
        s.save_matrix(&matrix()).unwrap();
    }
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    assert!(s.save_matrix(&matrix()).unwrap_err().contains("eight"));
    assert_eq!(before, serde_json::to_value(s.snapshot().unwrap()).unwrap());
    assert_eq!(s.snapshot().unwrap().unwrap().matrices.len(), 8);
}
#[test]
fn assessment_crud_target_and_cell_constraints() {
    let (mut s, _, top, threat) = fixture();
    let m = matrix();
    s.save_matrix(&m).unwrap();
    let consequence = s
        .add_entity(&top, "consequence", "Fire", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "consequence")
        .unwrap()
        .id;
    let mut a = Assessment {
        target_id: top.clone(),
        matrix_id: m.id.clone(),
        phase: "inherent".into(),
        likelihood_position: 1,
        severity_position: 1,
    };
    s.set_assessment(&a, false).unwrap();
    a.phase = "residual".into();
    a.likelihood_position = 0;
    s.set_assessment(&a, false).unwrap();
    a.severity_position = 0;
    s.set_assessment(&a, false).unwrap();
    assert_eq!(s.snapshot().unwrap().unwrap().assessments.len(), 2);
    a.target_id = consequence;
    s.set_assessment(&a, false).unwrap();
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    a.target_id = threat;
    assert!(s
        .set_assessment(&a, false)
        .unwrap_err()
        .contains("Only top events"));
    a.target_id = top;
    a.likelihood_position = 99;
    assert!(s.set_assessment(&a, false).is_err());
    assert_eq!(before, serde_json::to_value(s.snapshot().unwrap()).unwrap());
    a.likelihood_position = 0;
    s.set_assessment(&a, true).unwrap();
    assert_eq!(s.snapshot().unwrap().unwrap().assessments.len(), 2);
    s.clear_assessments().unwrap();
    assert!(s.snapshot().unwrap().unwrap().assessments.is_empty());
}
#[test]
fn soft_hide_coordinate_order_category_ranks_and_resize_restrict() {
    let (mut s, _, top, _) = fixture();
    let mut m = matrix();
    s.save_matrix(&m).unwrap();
    s.set_assessment(
        &Assessment {
            target_id: top,
            matrix_id: m.id.clone(),
            phase: "inherent".into(),
            likelihood_position: 1,
            severity_position: 1,
        },
        false,
    )
    .unwrap();
    m.title.clear();
    m.coordinate_order = "yx".into();
    m.categories[0].rank = 1;
    m.categories[1].rank = 0;
    s.save_matrix(&m).unwrap();
    let c = s.snapshot().unwrap().unwrap();
    assert!(c.matrices[0].title.is_empty());
    assert_eq!(c.matrices[0].coordinate_order, "yx");
    assert_eq!(c.matrices[0].categories[0].id, m.categories[1].id);
    assert_eq!(c.assessments.len(), 1);
    let before = serde_json::to_value(&c).unwrap();
    m.severity.pop();
    m.cells.retain(|c| c.severity_position == 0);
    assert!(s.save_matrix(&m).unwrap_err().contains("referenced"));
    assert_eq!(
        before,
        serde_json::to_value(s.snapshot().unwrap().unwrap()).unwrap()
    );
    assert!(s.delete_matrix(&m.id).unwrap_err().contains("assessment"));
    s.clear_assessments().unwrap();
    s.save_matrix(&m).unwrap();
    s.delete_matrix(&m.id).unwrap();
    assert!(s.snapshot().unwrap().unwrap().matrices.is_empty());
}
#[test]
fn typed_properties_effectiveness_and_reference_deletion() {
    let (mut s, _, _, threat) = fixture();
    let b = s
        .add_entity(&threat, "barrier", "Inspection", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "barrier")
        .unwrap()
        .id;
    let id = uuid::Uuid::new_v4().to_string();
    s.save_lookup(&Lookup {
        id: id.clone(),
        table_key: "effectiveness".into(),
        code: "E".into(),
        title: "Effective".into(),
        color: "#aaddaa".into(),
        sort_order: 0,
        metadata: json!({"description":"Test","frequency":0.2}),
    })
    .unwrap();
    s.edit_entity(
        &b,
        "Inspection",
        "Details",
        &json!({"effectiveness_id":id,"criteria_met":true}),
    )
    .unwrap();
    assert_eq!(
        s.snapshot()
            .unwrap()
            .unwrap()
            .tree
            .iter()
            .find(|n| n.id == b)
            .unwrap()
            .effectiveness_id
            .as_deref(),
        Some(id.as_str())
    );
    assert!(s.delete_lookup(&id).unwrap_err().contains("reference"));
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    assert!(s
        .edit_entity(&b, "Broken", "", &json!({"criteria_met":"yes"}))
        .is_err());
    assert!(s
        .edit_entity(&b, "Broken", "", &json!({"effectiveness_id":threat}))
        .is_err());
    assert_eq!(before, serde_json::to_value(s.snapshot().unwrap()).unwrap());
    s.edit_entity(&b, "Inspection", "", &json!({"effectiveness_id":""}))
        .unwrap();
    s.delete_lookup(&id).unwrap();
}
#[test]
fn action_register_matches_tree_and_atomic_inline_edits() {
    let (mut s, root, _, threat) = fixture();
    let role = s
        .add_entity(&root, "job_title", "Operator", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "job_title")
        .unwrap()
        .id;
    let c = s
        .add_entity(
            &threat,
            "action",
            "Improve hose",
            "Initial",
            &json!({"completed":false,"target_date":"2026-10-20","links":{"action_party":[role]}}),
        )
        .unwrap();
    let id = c.actions[0].id.clone();
    assert_eq!(c.actions.len(), 1);
    assert_eq!(c.actions[0].parent_title, "Hose rupture");
    assert_eq!(c.actions[0].responsible, vec!["Operator"]);
    s.edit_entity(
        &id,
        "Improve hose revised",
        "Edited",
        &json!({"completed":true,"target_date":"2026-10-21","sign_off_comment":"Reviewed"}),
    )
    .unwrap();
    let c = s.snapshot().unwrap().unwrap();
    assert_eq!(c.actions[0].properties["completed"], true);
    assert_eq!(c.actions[0].description, "Edited");
    assert_eq!(
        c.tree.iter().find(|n| n.id == id).unwrap().title,
        c.actions[0].title
    );
    let before = serde_json::to_value(&c).unwrap();
    assert!(s
        .edit_entity(&id, "Broken", "", &json!({"target_date":"2026-02-30"}))
        .is_err());
    assert!(s
        .edit_entity(
            &id,
            "Broken",
            "",
            &json!({"links":{"action_party":[threat]}})
        )
        .is_err());
    assert_eq!(
        before,
        serde_json::to_value(s.snapshot().unwrap().unwrap()).unwrap()
    );
}
#[test]
fn custom_fields_are_typed_and_kind_wide_rename_delete() {
    let (mut s, _, _, threat) = fixture();
    s.custom_field("threat", "Frequency estimate", "number", None, false)
        .unwrap();
    s.edit_entity(
        &threat,
        "Hose rupture",
        "",
        &json!({"user_data":{"Frequency estimate":0.5}}),
    )
    .unwrap();
    assert!(s
        .edit_entity(
            &threat,
            "Broken",
            "",
            &json!({"user_data":{"Frequency estimate":"wrong"}})
        )
        .is_err());
    s.custom_field(
        "threat",
        "Estimate",
        "number",
        Some("Frequency estimate"),
        false,
    )
    .unwrap();
    let c = s.snapshot().unwrap().unwrap();
    assert_eq!(c.custom_fields["threat"]["Estimate"], "number");
    assert_eq!(
        c.tree.iter().find(|n| n.id == threat).unwrap().properties["user_data"]["Estimate"],
        0.5
    );
    s.custom_field("threat", "Estimate", "number", Some("Estimate"), true)
        .unwrap();
    assert!(s
        .snapshot()
        .unwrap()
        .unwrap()
        .tree
        .iter()
        .find(|n| n.id == threat)
        .unwrap()
        .properties["user_data"]
        .as_object()
        .unwrap()
        .is_empty());
}
#[test]
fn all_phase3_data_round_trip_on_unchanged_schema() {
    let (mut s, _, top, threat) = fixture();
    let m = matrix();
    s.save_matrix(&m).unwrap();
    s.set_assessment(
        &Assessment {
            target_id: top,
            matrix_id: m.id.clone(),
            phase: "residual".into(),
            likelihood_position: 0,
            severity_position: 0,
        },
        false,
    )
    .unwrap();
    s.add_entity(
        &threat,
        "action",
        "Improve",
        "",
        &json!({"target_date":"2026-11-01","completed":false}),
    )
    .unwrap();
    let c = s.snapshot().unwrap().unwrap();
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("risk.sqlite");
    s.save(Some(&path)).unwrap();
    s.close();
    let reopened = s.open(&path).unwrap();
    assert_eq!(
        serde_json::to_value(c.tree).unwrap(),
        serde_json::to_value(reopened.tree).unwrap()
    );
    assert_eq!(
        serde_json::to_value(c.matrices).unwrap(),
        serde_json::to_value(reopened.matrices).unwrap()
    );
    assert_eq!(
        serde_json::to_value(c.assessments).unwrap(),
        serde_json::to_value(reopened.assessments).unwrap()
    );
    assert_eq!(
        serde_json::to_value(c.actions).unwrap(),
        serde_json::to_value(reopened.actions).unwrap()
    );
}
