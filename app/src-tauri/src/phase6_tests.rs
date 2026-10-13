use crate::{audit::*, case::CaseStore};
use rusqlite::params;
use serde_json::json;
fn q(id: &str, parent: Option<&str>, targets: Vec<String>) -> Question {
    Question {
        id: id.into(),
        parent_id: parent.map(str::to_string),
        title: format!("Review {id}"),
        criteria: "Check the local inspection record".into(),
        answers: vec![
            "Confirmed".into(),
            "Needs review".into(),
            "Not established".into(),
        ],
        positive_first: true,
        most_negative: false,
        contextual: false,
        targets,
    }
}
fn fixture() -> (CaseStore, Audit, String, String, String) {
    let mut s = CaseStore::default();
    let root = s.create("Transfer audit", "Workshop").unwrap().tree[0]
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
    let snap = s.add_diagram(&group, "Liquid transfer", "Release").unwrap();
    let top = snap
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
    let b = s
        .add_entity(&threat, "barrier", "Inspection", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "barrier")
        .unwrap()
        .id;
    let act = s
        .add_entity(&root, "activity", "Inspection work", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "activity")
        .unwrap()
        .id;
    let doc = s
        .add_entity(&root, "document", "Checklist", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "document")
        .unwrap()
        .id;
    s.link(&b, &act, "activity", false).unwrap();
    s.link(&act, &doc, "document", false).unwrap();
    let a = Audit {
        id: "audit-one".into(),
        title: "Safeguard review".into(),
        saved_filters: vec![],
        questions: vec![
            q("root", None, vec![]),
            q("barrier", Some("root"), vec![b.clone()]),
            q("activity", None, vec![act.clone()]),
            q("document", None, vec![doc.clone()]),
        ],
        surveys: vec![Survey {
            id: "survey-one".into(),
            title: "Transfer checks".into(),
            code: "T01".into(),
            description: "Routine qualitative review".into(),
            question_ids: vec!["barrier".into(), "activity".into(), "document".into()],
            fillouts: vec![],
        }],
    };
    s.save_audit(&a, None).unwrap();
    let a = s.snapshot().unwrap().unwrap().audits.remove(0);
    (s, a, b, threat, act)
}
fn fill(a: &Audit, id: &str, choice: usize) -> Fillout {
    Fillout {
        id: id.into(),
        date: "2026-10-13".into(),
        respondent: format!("Reviewer {id}"),
        unit: "Operations".into(),
        completed: true,
        answers: expected(a, &a.surveys[0])
            .unwrap()
            .into_iter()
            .map(|(question_id, context_id)| Answer {
                question_id,
                context_id,
                choice: Some(choice),
                remark: "Inspection register checked".into(),
            })
            .collect(),
    }
}
#[test]
fn cycles_target_kinds_and_same_audit_constraints_remain_authoritative() {
    let (mut s, a, b, threat, _) = fixture();
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    let db = &s.active.as_ref().unwrap().db;
    assert!(db
        .execute(
            "UPDATE audit_question SET parent_id='barrier' WHERE id='root'",
            []
        )
        .is_err());
    assert!(db.execute("INSERT INTO audit_question(id,audit_id,parent_id,title) VALUES('self','audit-one','self','Self')",[]).is_err());
    assert!(db
        .execute(
            "UPDATE audit_question_target SET target_id=? WHERE question_id='barrier'",
            [&threat]
        )
        .is_err());
    assert!(db
        .execute(
            "INSERT INTO audit_question_target VALUES('root',?)",
            [&threat]
        )
        .is_err());
    assert!(db
        .execute(
            "INSERT INTO audit_survey_question VALUES('survey-one','other','root',9)",
            []
        )
        .is_err());
    assert_eq!(
        db.query_row(
            "SELECT target_id FROM audit_question_target WHERE question_id='barrier'",
            [],
            |r| r.get::<_, String>(0)
        )
        .unwrap(),
        b
    );
    let mut cycle = a.clone();
    cycle
        .questions
        .iter_mut()
        .find(|q| q.id == "root")
        .unwrap()
        .parent_id = Some("barrier".into());
    assert!(s.save_audit(&cycle, Some(&a)).is_err());
    let mut invalid = a.clone();
    invalid.questions[0].targets.push(threat);
    assert!(s.save_audit(&invalid, Some(&a)).is_err());
    assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
}
#[test]
fn survey_crud_contextual_fillouts_completion_and_roundtrip() {
    let (mut s, old, b, _, act) = fixture();
    let mut a = old.clone();
    let question = a.questions.iter_mut().find(|q| q.id == "barrier").unwrap();
    question.contextual = true;
    question.targets.push(act.clone());
    let response = fill(&a, "one", 0);
    a.surveys[0].fillouts.push(response);
    s.save_audit(&a, Some(&old)).unwrap();
    let a = s.snapshot().unwrap().unwrap().audits.remove(0);
    assert_eq!(a.surveys[0].fillouts[0].answers.len(), 4);
    let folder = tempfile::tempdir().unwrap();
    let file = folder.path().join("audit.sqlite");
    s.save(Some(&file)).unwrap();
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    let bytes = std::fs::read(&file).unwrap();
    let mut invalid = a.clone();
    invalid.surveys[0].fillouts[0].answers[0].choice = None;
    assert!(s.save_audit(&invalid, Some(&a)).is_err());
    invalid = a.clone();
    invalid.surveys[0].fillouts[0].answers[0].context_id = Some(b.clone());
    let duplicate = invalid.surveys[0].fillouts[0].answers[0].clone();
    invalid.surveys[0].fillouts[0].answers.push(duplicate);
    assert!(s.save_audit(&invalid, Some(&a)).is_err());
    invalid = a.clone();
    invalid.surveys[0].fillouts[0].answers[0].choice = Some(999);
    assert!(s.save_audit(&invalid, Some(&a)).is_err());
    assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
    assert_eq!(std::fs::read(&file).unwrap(), bytes);
    s.close();
    let reopened = s.open(&file).unwrap();
    assert_eq!(
        serde_json::to_value(reopened.audits).unwrap(),
        serde_json::to_value(&a).map(|v| json!([v])).unwrap()
    );
    let mut changed = a.clone();
    changed.surveys[0].title = "Edited survey".into();
    changed.surveys[0].description = "Follow-up".into();
    s.save_audit(&changed, Some(&a)).unwrap();
    assert!(s.save_audit(&a, Some(&a)).is_err());
    let current = s.snapshot().unwrap().unwrap().audits.remove(0);
    let mut removed = current.clone();
    removed.surveys.clear();
    s.save_audit(&removed, Some(&current)).unwrap();
    let current = s.snapshot().unwrap().unwrap().audits.remove(0);
    s.delete_audit(&current.id, &current).unwrap();
    assert!(s.snapshot().unwrap().unwrap().audits.is_empty());
}
#[test]
fn ordinal_centers_keying_and_textual_eight_answer_examples() {
    let mut question = q("q", None, vec![]);
    let choices = scale(&question).unwrap();
    assert_eq!(
        choices
            .iter()
            .map(|c| c.tendency.as_str())
            .collect::<Vec<_>>(),
        vec!["positive", "neutral", "negative"]
    );
    question.positive_first = false;
    assert_eq!(
        scale(&question)
            .unwrap()
            .iter()
            .map(|c| c.tendency.as_str())
            .collect::<Vec<_>>(),
        vec!["negative", "neutral", "positive"]
    );
    question.answers = vec![
        "Missing".into(),
        "@".into(),
        "Developing".into(),
        "Established".into(),
        "Verified".into(),
    ];
    assert_eq!(
        scale(&question)
            .unwrap()
            .iter()
            .map(|c| c.tendency.as_str())
            .collect::<Vec<_>>(),
        vec!["negative", "positive", "positive", "positive"]
    );
    question.answers = vec!["Missing".into(), "@Developing".into(), "Established".into()];
    assert_eq!(scale(&question).unwrap()[1].tendency, "neutral");
    question.answers.push("@Other".into());
    assert!(scale(&question).is_err());
    let (mut s, old, _, _, _) = fixture();
    let mut a = old.clone();
    a.surveys[0].fillouts = (0..8).map(|i| fill(&a, &i.to_string(), 0)).collect();
    s.save_audit(&a, Some(&old)).unwrap();
    let rows = s.audit_results(&a.id, &ResultFilter::default()).unwrap();
    assert_eq!(
        rows.iter()
            .find(|r| r.question_id == "barrier")
            .unwrap()
            .counts,
        vec![8, 0, 0]
    );
    // SPEC021 §24.10.2: n=8 fully positive, fully neutral, and balanced negative/positive.
    let old = s.snapshot().unwrap().unwrap().audits.remove(0);
    let mut a = old.clone();
    for f in &mut a.surveys[0].fillouts {
        for r in &mut f.answers {
            r.choice = Some(1);
        }
    }
    s.save_audit(&a, Some(&old)).unwrap();
    assert_eq!(
        s.audit_results(&a.id, &ResultFilter::default())
            .unwrap()
            .iter()
            .find(|r| r.question_id == "barrier")
            .unwrap()
            .counts,
        vec![0, 8, 0]
    );
    let old = s.snapshot().unwrap().unwrap().audits.remove(0);
    let mut a = old.clone();
    for (i, f) in a.surveys[0].fillouts.iter_mut().enumerate() {
        for r in &mut f.answers {
            r.choice = Some(if i < 4 { 0 } else { 2 });
        }
    }
    s.save_audit(&a, Some(&old)).unwrap();
    assert_eq!(
        s.audit_results(&a.id, &ResultFilter::default())
            .unwrap()
            .iter()
            .find(|r| r.question_id == "barrier")
            .unwrap()
            .counts,
        vec![4, 0, 4]
    );
}
#[test]
fn filters_propagation_most_negative_and_scale_edit_protection() {
    let (mut s, old, b, _, _) = fixture();
    let mut a = old.clone();
    a.questions
        .iter_mut()
        .find(|q| q.id == "barrier")
        .unwrap()
        .most_negative = true;
    a.surveys[0].fillouts = vec![fill(&a, "early", 0), fill(&a, "later", 2)];
    a.surveys[0].fillouts[1].date = "2026-10-14".into();
    a.surveys[0].fillouts[1].unit = "Night shift".into();
    s.save_audit(&a, Some(&old)).unwrap();
    let rows = s
        .audit_results(
            &a.id,
            &ResultFilter {
                target_id: Some(b.clone()),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(
        rows.iter()
            .find(|r| r.question_id == "barrier")
            .unwrap()
            .worst_choice,
        Some(2)
    );
    let rows = s
        .audit_results(
            &a.id,
            &ResultFilter {
                unit: Some("Operations".into()),
                from: Some("2026-10-13".into()),
                to: Some("2026-10-13".into()),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(
        rows.iter()
            .find(|r| r.question_id == "barrier")
            .unwrap()
            .counts,
        vec![1, 0, 0]
    );
    assert!(s
        .audit_results(
            &a.id,
            &ResultFilter {
                negative_only: true,
                unit: Some("Operations".into()),
                ..Default::default()
            }
        )
        .unwrap()
        .is_empty());
    let old = s.snapshot().unwrap().unwrap().audits.remove(0);
    let mut bad = old.clone();
    bad.questions[0].answers.reverse();
    assert!(s.save_audit(&bad, Some(&old)).is_err());
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    let db = &s.active.as_ref().unwrap().db;
    let raw: String = db
        .query_row("SELECT metadata_json FROM audit WHERE id=?", [&a.id], |r| {
            r.get(0)
        })
        .unwrap();
    let mut corrupt: serde_json::Value = serde_json::from_str(&raw).unwrap();
    corrupt["surveys"]["survey-one"]["fillouts"][0]["answers"][0]["questionId"] = json!("unknown");
    db.execute(
        "UPDATE audit SET metadata_json=? WHERE id=?",
        params![corrupt.to_string(), a.id],
    )
    .unwrap();
    assert!(crate::audit::validate(db).is_err());
    db.execute(
        "UPDATE audit SET metadata_json=? WHERE id=?",
        params![raw, a.id],
    )
    .unwrap();
    assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
}
#[test]
fn core_table_roundtrip_preserves_all_audit_data() {
    let (mut s, old, _, _, _) = fixture();
    let mut a = old.clone();
    let response = fill(&a, "one", 1);
    a.surveys[0].fillouts.push(response);
    s.save_audit(&a, Some(&old)).unwrap();
    let text = s.table_export().unwrap().text;
    let mut target = CaseStore::default();
    target.create("New", "Root").unwrap();
    let preview = target.import_preview(&text).unwrap();
    assert!(preview.errors.is_empty(), "{:?}", preview.errors);
    target.import_apply(&text, &preview.signature).unwrap();
    assert_eq!(target.table_export().unwrap().text, text);
    assert_eq!(
        serde_json::to_value(target.snapshot().unwrap().unwrap().audits).unwrap(),
        serde_json::to_value(s.snapshot().unwrap().unwrap().audits).unwrap()
    );
}
#[test]
fn saved_filters_persist_validate_references_and_protect_deletion() {
    let (mut s, old, _, _, _) = fixture();
    let root = s
        .snapshot()
        .unwrap()
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.kind == "organization")
        .unwrap()
        .id;
    let target = s
        .add_entity(&root, "document", "Review scope", "", &json!({}))
        .unwrap()
        .tree
        .into_iter()
        .find(|n| n.title == "Review scope")
        .unwrap()
        .id;
    let mut a = old.clone();
    a.saved_filters.push(SavedFilter {
        id: "profile".into(),
        name: "Inspection view".into(),
        filter: ResultFilter {
            survey_id: Some(a.surveys[0].id.clone()),
            target_id: Some(target.clone()),
            from: Some("2026-10-01".into()),
            ..Default::default()
        },
        question_id: Some("barrier".into()),
        show_questions: true,
    });
    s.save_audit(&a, Some(&old)).unwrap();
    assert!(s.delete_entity(&target).is_err());
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("filters.sqlite");
    s.save(Some(&file)).unwrap();
    s.close();
    let restored = s.open(&file).unwrap();
    assert_eq!(restored.audits[0].saved_filters, a.saved_filters);
    let old = restored.audits[0].clone();
    let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
    let mut invalid = old.clone();
    invalid.saved_filters[0].filter.survey_id = Some("other".into());
    assert!(s.save_audit(&invalid, Some(&old)).is_err());
    invalid = old.clone();
    invalid.saved_filters[0].question_id = Some("other".into());
    assert!(s.save_audit(&invalid, Some(&old)).is_err());
    assert_eq!(serde_json::to_value(s.snapshot().unwrap()).unwrap(), before);
    let mut clean = old.clone();
    clean.saved_filters.clear();
    s.save_audit(&clean, Some(&old)).unwrap();
    assert!(s.delete_entity(&target).is_ok());
}
