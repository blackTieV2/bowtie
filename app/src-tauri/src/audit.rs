//! Original audit services: ordinal choices, constrained associations and persisted fillouts.
use crate::case::{CaseSnapshot, CaseStore};
use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Question {
    pub id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub criteria: String,
    pub answers: Vec<String>,
    pub positive_first: bool,
    pub most_negative: bool,
    pub contextual: bool,
    pub targets: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Answer {
    pub question_id: String,
    pub context_id: Option<String>,
    pub choice: Option<usize>,
    pub remark: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fillout {
    pub id: String,
    pub date: String,
    pub respondent: String,
    pub unit: String,
    pub completed: bool,
    pub answers: Vec<Answer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Survey {
    pub id: String,
    pub title: String,
    pub code: String,
    pub description: String,
    pub question_ids: Vec<String>,
    pub fillouts: Vec<Fillout>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Audit {
    pub id: String,
    pub title: String,
    pub questions: Vec<Question>,
    pub surveys: Vec<Survey>,
    #[serde(default)]
    pub saved_filters: Vec<SavedFilter>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SavedFilter {
    pub id: String,
    pub name: String,
    pub filter: ResultFilter,
    pub question_id: Option<String>,
    pub show_questions: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct QuestionMeta {
    criteria: String,
    positive_first: bool,
    most_negative: bool,
    contextual: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SurveyMeta {
    code: String,
    description: String,
    fillouts: Vec<Fillout>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuditMeta {
    version: u32,
    surveys: BTreeMap<String, SurveyMeta>,
    #[serde(default)]
    filters: Vec<SavedFilter>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Choice {
    pub label: String,
    pub tendency: String,
}
pub fn scale(question: &Question) -> Result<Vec<Choice>> {
    let mut labels = Vec::new();
    let mut center = None;
    for raw in &question.answers {
        if raw.trim().is_empty() || raw.contains('\n') || raw.contains('\r') {
            return Err("Each ordinal answer must occupy one nonempty line".into());
        }
        if let Some(label) = raw.strip_prefix('@') {
            if center.is_some() {
                return Err("Use at most one answer-scale center marker".into());
            }
            center = Some(if label.is_empty() {
                labels.len() as f64 - 0.5
            } else {
                labels.len() as f64
            });
            if label.is_empty() {
                continue;
            }
            if label.contains('@') {
                return Err("Use a single leading center marker".into());
            }
            labels.push(label.to_string());
        } else {
            if raw.contains('@') {
                return Err("The center marker must be the first character of a line".into());
            }
            labels.push(raw.clone());
        }
    }
    if labels.is_empty()
        || labels.iter().any(|s| s.trim().is_empty())
        || labels.iter().collect::<BTreeSet<_>>().len() != labels.len()
    {
        return Err("Provide distinct, nonempty ordinal answer choices".into());
    }
    let center = center.unwrap_or((labels.len() as f64 - 1.0) / 2.0);
    Ok(labels
        .into_iter()
        .enumerate()
        .map(|(i, label)| Choice {
            label,
            tendency: if i as f64 == center {
                "neutral"
            } else if ((i as f64) < center) == question.positive_first {
                "positive"
            } else {
                "negative"
            }
            .into(),
        })
        .collect())
}
pub fn expected(audit: &Audit, survey: &Survey) -> Result<Vec<(String, Option<String>)>> {
    let mut result = Vec::new();
    for id in &survey.question_ids {
        let q = audit
            .questions
            .iter()
            .find(|q| &q.id == id)
            .ok_or("Survey question belongs to another audit or does not exist")?;
        if q.contextual && !q.targets.is_empty() {
            for target in &q.targets {
                result.push((id.clone(), Some(target.clone())));
            }
        } else {
            result.push((id.clone(), None));
        }
    }
    Ok(result)
}
fn bundle_valid(a: &Audit) -> Result<()> {
    if a.id.trim().is_empty() || a.title.trim().is_empty() {
        return Err("Audit identity and name are required".into());
    }
    let mut ids = BTreeSet::new();
    for q in &a.questions {
        if q.id.trim().is_empty() || q.title.trim().is_empty() || !ids.insert(&q.id) {
            return Err("Question names and distinct identities are required".into());
        }
        scale(q)?;
        if q.targets.iter().collect::<BTreeSet<_>>().len() != q.targets.len() {
            return Err("Question targets must be distinct".into());
        }
    }
    for q in &a.questions {
        let mut cursor = Some(&q.id);
        let mut visited = BTreeSet::new();
        while let Some(id) = cursor {
            if !visited.insert(id) {
                return Err("Question hierarchy cycle".into());
            }
            let parent = a
                .questions
                .iter()
                .find(|n| &n.id == id)
                .ok_or("Question parent must belong to this audit")?;
            cursor = parent.parent_id.as_ref();
        }
    }
    let mut surveys = BTreeSet::new();
    let mut fillouts = BTreeSet::new();
    for s in &a.surveys {
        if s.id.trim().is_empty()
            || s.title.trim().is_empty()
            || s.code.trim().is_empty()
            || !surveys.insert(&s.id)
        {
            return Err("Survey name, code and distinct identity are required".into());
        }
        if s.question_ids.iter().collect::<BTreeSet<_>>().len() != s.question_ids.len() {
            return Err("A survey is a flat list of distinct questions".into());
        }
        let required = expected(a, s)?.into_iter().collect::<BTreeSet<_>>();
        if required.is_empty() && !s.fillouts.is_empty() {
            return Err("Select questions before conducting a survey".into());
        }
        for f in &s.fillouts {
            if f.id.trim().is_empty()
                || !fillouts.insert(&f.id)
                || f.respondent.trim().is_empty()
                || !crate::properties::valid_date(&f.date)
            {
                return Err("Fillouts need a distinct identity, respondent and valid date".into());
            }
            let actual = f
                .answers
                .iter()
                .map(|r| (r.question_id.clone(), r.context_id.clone()))
                .collect::<BTreeSet<_>>();
            if actual != required || actual.len() != f.answers.len() {
                return Err("Each fillout must contain exactly one answer slot per selected question/context".into());
            }
            for r in &f.answers {
                let q = a
                    .questions
                    .iter()
                    .find(|q| q.id == r.question_id)
                    .ok_or("Unknown response question")?;
                if r.choice
                    .is_some_and(|index| index >= scale(q).map(|v| v.len()).unwrap_or(0))
                {
                    return Err("Response is not an ordinal answer choice".into());
                }
                if f.completed && r.choice.is_none() {
                    return Err("Answer every question/context before completing a fillout".into());
                }
            }
        }
    }
    let mut filters = BTreeSet::new();
    for saved in &a.saved_filters {
        if saved.id.trim().is_empty() || saved.name.trim().is_empty() || !filters.insert(&saved.id)
        {
            return Err("Saved filters need distinct identities and names".into());
        }
        if saved
            .question_id
            .as_ref()
            .is_some_and(|id| !a.questions.iter().any(|q| &q.id == id))
        {
            return Err("Saved filter refers to a missing question".into());
        }
        filter_valid(a, &saved.filter)?;
    }
    Ok(())
}
pub fn audits(db: &Connection) -> Result<Vec<Audit>> {
    let mut query = db
        .prepare("SELECT id,title,metadata_json FROM audit ORDER BY title,id")
        .map_err(err)?;
    let basics = query
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
    let mut result = Vec::new();
    for (id, title, metadata) in basics {
        let meta: AuditMeta = serde_json::from_str(&metadata).map_err(err)?;
        if meta.version != 1 {
            return Err("Unsupported audit data version".into());
        }
        let questions = {
            let mut query=db.prepare("SELECT id,parent_id,title,answers_json,metadata_json FROM audit_question WHERE audit_id=? ORDER BY title,id").map_err(err)?;
            let rows = query
                .query_map([&id], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, Option<String>>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, String>(4)?,
                    ))
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            let mut out = Vec::new();
            for (qid, parent_id, title, answers, metadata) in rows {
                let qm: QuestionMeta = serde_json::from_str(&metadata).map_err(err)?;
                let mut query=db.prepare("SELECT target_id FROM audit_question_target WHERE question_id=? ORDER BY target_id").map_err(err)?;
                let targets = query
                    .query_map([&qid], |r| r.get(0))
                    .map_err(err)?
                    .collect::<rusqlite::Result<Vec<String>>>()
                    .map_err(err)?;
                out.push(Question {
                    id: qid,
                    parent_id,
                    title,
                    criteria: qm.criteria,
                    answers: serde_json::from_str(&answers).map_err(err)?,
                    positive_first: qm.positive_first,
                    most_negative: qm.most_negative,
                    contextual: qm.contextual,
                    targets,
                });
            }
            out
        };
        let surveys = {
            let mut query = db
                .prepare("SELECT id,title FROM audit_survey WHERE audit_id=? ORDER BY title,id")
                .map_err(err)?;
            let rows = query
                .query_map([&id], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            let mut out = Vec::new();
            for (sid, title) in rows {
                let sm = meta.surveys.get(&sid).ok_or("Survey metadata is missing")?;
                let mut query=db.prepare("SELECT question_id,position FROM audit_survey_question WHERE survey_id=? ORDER BY position").map_err(err)?;
                let members = query
                    .query_map([&sid], |r| {
                        Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
                    })
                    .map_err(err)?
                    .collect::<rusqlite::Result<Vec<_>>>()
                    .map_err(err)?;
                if members
                    .iter()
                    .enumerate()
                    .any(|(i, (_, pos))| i as i64 != *pos)
                {
                    return Err("Survey positions must be contiguous".into());
                }
                out.push(Survey {
                    id: sid,
                    title,
                    code: sm.code.clone(),
                    description: sm.description.clone(),
                    question_ids: members.into_iter().map(|(id, _)| id).collect(),
                    fillouts: sm.fillouts.clone(),
                });
            }
            if out.len() != meta.surveys.len() {
                return Err("Audit metadata references an unknown survey".into());
            }
            out
        };
        result.push(Audit {
            id,
            title,
            questions,
            surveys,
            saved_filters: meta.filters,
        });
    }
    Ok(result)
}
pub fn validate(db: &Connection) -> Result<()> {
    for audit in audits(db)? {
        bundle_valid(&audit)?;
        for saved in &audit.saved_filters {
            if let Some(id) = &saved.filter.target_id {
                let exists:bool=db.query_row("SELECT EXISTS(SELECT 1 FROM entity WHERE id=? AND kind IN ('barrier','activity','document'))",[id],|r|r.get(0)).map_err(err)?;
                if !exists {
                    return Err(
                        "Saved audit filter target does not exist or has an invalid kind".into(),
                    );
                }
            }
        }
        for q in &audit.questions {
            for target in &q.targets {
                let valid:bool=db.query_row("SELECT EXISTS(SELECT 1 FROM entity WHERE id=? AND kind IN ('barrier','activity','document'))",[target],|r|r.get(0)).map_err(err)?;
                if !valid {
                    return Err("Audit targets require a barrier, activity or document".into());
                }
            }
        }
    }
    Ok(())
}
pub fn summaries(db: &Connection) -> Result<BTreeMap<String, Vec<ResultRow>>> {
    audits(db)?
        .iter()
        .map(|a| Ok((a.id.clone(), results(a, &ResultFilter::default(), db)?)))
        .collect()
}
fn clear_children(tx: &Transaction<'_>, id: &str) -> Result<()> {
    tx.execute("DELETE FROM audit_survey_question WHERE audit_id=?", [id])
        .map_err(err)?;
    tx.execute("DELETE FROM audit_survey WHERE audit_id=?", [id])
        .map_err(err)?;
    tx.execute("DELETE FROM audit_question_target WHERE question_id IN (SELECT id FROM audit_question WHERE audit_id=?)",[id]).map_err(err)?;
    loop {
        let remaining: i64 = tx
            .query_row(
                "SELECT count(*) FROM audit_question WHERE audit_id=?",
                [id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if remaining == 0 {
            break;
        }
        let deleted=tx.execute("DELETE FROM audit_question WHERE audit_id=? AND NOT EXISTS(SELECT 1 FROM audit_question child WHERE child.parent_id=audit_question.id)",[id]).map_err(err)?;
        if deleted == 0 {
            return Err("Existing question hierarchy cannot be safely removed".into());
        }
    }
    Ok(())
}
pub(crate) fn write_bundle(tx: &Transaction<'_>, a: &Audit) -> Result<()> {
    bundle_valid(a)?;
    for q in &a.questions {
        let owners: i64 = tx
            .query_row(
                "SELECT count(*) FROM audit_question WHERE id=? AND audit_id<>?",
                params![q.id, a.id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if owners > 0 {
            return Err("Question identity belongs to another audit".into());
        }
    }
    for s in &a.surveys {
        let owners: i64 = tx
            .query_row(
                "SELECT count(*) FROM audit_survey WHERE id=? AND audit_id<>?",
                params![s.id, a.id],
                |r| r.get(0),
            )
            .map_err(err)?;
        if owners > 0 {
            return Err("Survey identity belongs to another audit".into());
        }
    }
    // Recorded answers retain their ordinal interpretation; unsafe option/keying edits
    // are rejected rather than reclassifying historical responses.
    if let Some(old) = audits(tx)?.into_iter().find(|old| old.id == a.id) {
        for q in &old.questions {
            let answered = old
                .surveys
                .iter()
                .flat_map(|s| &s.fillouts)
                .flat_map(|f| &f.answers)
                .any(|r| r.question_id == q.id && r.choice.is_some());
            if answered {
                if let Some(new) = a.questions.iter().find(|n| n.id == q.id) {
                    if q.answers != new.answers
                        || q.positive_first != new.positive_first
                        || q.contextual != new.contextual
                    {
                        return Err("Recorded answers prevent changing their scale/keying/context mode; remove those fillouts first".into());
                    }
                }
            }
        }
    }
    clear_children(tx, &a.id)?;
    let metadata = AuditMeta {
        version: 1,
        filters: a.saved_filters.clone(),
        surveys: a
            .surveys
            .iter()
            .map(|s| {
                (
                    s.id.clone(),
                    SurveyMeta {
                        code: s.code.clone(),
                        description: s.description.clone(),
                        fillouts: s.fillouts.clone(),
                    },
                )
            })
            .collect(),
    };
    tx.execute("INSERT INTO audit(id,title,metadata_json) VALUES(?,?,?) ON CONFLICT(id) DO UPDATE SET title=excluded.title,metadata_json=excluded.metadata_json",params![a.id,a.title,serde_json::to_string(&metadata).map_err(err)?]).map_err(err)?;
    let mut pending = a.questions.iter().collect::<Vec<_>>();
    let mut inserted = BTreeSet::new();
    while !pending.is_empty() {
        let mut rest = Vec::new();
        for q in pending {
            if q.parent_id
                .as_ref()
                .is_some_and(|id| !inserted.contains(id))
            {
                rest.push(q);
                continue;
            }
            let meta = QuestionMeta {
                criteria: q.criteria.clone(),
                positive_first: q.positive_first,
                most_negative: q.most_negative,
                contextual: q.contextual,
            };
            tx.execute("INSERT INTO audit_question(id,audit_id,parent_id,title,answers_json,metadata_json) VALUES(?,?,?,?,?,?)",params![q.id,a.id,q.parent_id,q.title,serde_json::to_string(&q.answers).map_err(err)?,serde_json::to_string(&meta).map_err(err)?]).map_err(err)?;
            for target in &q.targets {
                tx.execute(
                    "INSERT INTO audit_question_target VALUES(?,?)",
                    params![q.id, target],
                )
                .map_err(err)?;
            }
            inserted.insert(q.id.clone());
        }
        pending = rest;
    }
    for s in &a.surveys {
        tx.execute(
            "INSERT INTO audit_survey(id,audit_id,title) VALUES(?,?,?)",
            params![s.id, a.id, s.title],
        )
        .map_err(err)?;
        for (i, q) in s.question_ids.iter().enumerate() {
            tx.execute("INSERT INTO audit_survey_question(survey_id,audit_id,question_id,position) VALUES(?,?,?,?)",params![s.id,a.id,q,i as i64]).map_err(err)?;
        }
    }
    validate(tx)
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResultFilter {
    pub survey_id: Option<String>,
    pub unit: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub fillout_id: Option<String>,
    pub target_id: Option<String>,
    pub negative_only: bool,
}
fn filter_valid(a: &Audit, filter: &ResultFilter) -> Result<()> {
    for date in [&filter.from, &filter.to].into_iter().flatten() {
        if !crate::properties::valid_date(date) {
            return Err("Invalid result date filter".into());
        }
    }
    if filter
        .from
        .as_ref()
        .zip(filter.to.as_ref())
        .is_some_and(|(a, b)| a > b)
    {
        return Err("The result date range is reversed".into());
    }
    if filter
        .survey_id
        .as_ref()
        .is_some_and(|id| !a.surveys.iter().any(|s| &s.id == id))
    {
        return Err("Result filter survey does not exist".into());
    }
    if let Some(id) = &filter.fillout_id {
        if !a
            .surveys
            .iter()
            .filter(|s| filter.survey_id.as_ref().is_none_or(|sid| &s.id == sid))
            .any(|s| s.fillouts.iter().any(|f| &f.id == id))
        {
            return Err(
                "Result filter fill-out does not belong to the selected survey/audit".into(),
            );
        }
    }
    Ok(())
}
pub fn entity_filter_references(db: &Connection, id: &str) -> Result<usize> {
    Ok(audits(db)?
        .iter()
        .flat_map(|a| &a.saved_filters)
        .filter(|saved| saved.filter.target_id.as_deref() == Some(id))
        .count())
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultRow {
    pub question_id: String,
    pub title: String,
    pub choices: Vec<Choice>,
    pub counts: Vec<usize>,
    pub unanswered: usize,
    pub worst_choice: Option<usize>,
    pub most_negative: bool,
    pub details: Vec<Value>,
}
pub fn results(a: &Audit, filter: &ResultFilter, db: &Connection) -> Result<Vec<ResultRow>> {
    filter_valid(a, filter)?;
    for date in [&filter.from, &filter.to].into_iter().flatten() {
        if !crate::properties::valid_date(date) {
            return Err("Invalid result date filter".into());
        }
    }
    if filter
        .from
        .as_ref()
        .zip(filter.to.as_ref())
        .is_some_and(|(a, b)| a > b)
    {
        return Err("The result date range is reversed".into());
    }
    let targets = if let Some(target) = &filter.target_id {
        let mut set = BTreeSet::from([target.clone()]);
        let mut query=db.prepare("SELECT target_id FROM entity_link WHERE source_id=? AND relation IN ('activity','document') UNION SELECT d.target_id FROM entity_link a JOIN entity_link d ON d.source_id=a.target_id WHERE a.source_id=? AND a.relation='activity' AND d.relation='document'").map_err(err)?;
        for value in query
            .query_map(params![target, target], |r| r.get::<_, String>(0))
            .map_err(err)?
        {
            set.insert(value.map_err(err)?);
        }
        Some(set)
    } else {
        None
    };
    let mut out = Vec::new();
    for q in &a.questions {
        if targets
            .as_ref()
            .is_some_and(|set| !q.targets.iter().any(|id| set.contains(id)))
        {
            continue;
        }
        let choices = scale(q)?;
        let mut counts = vec![0; choices.len()];
        let mut unanswered = 0;
        let mut details = Vec::new();
        for s in a
            .surveys
            .iter()
            .filter(|s| filter.survey_id.as_ref().is_none_or(|id| &s.id == id))
        {
            for f in s.fillouts.iter().filter(|f| {
                filter.unit.as_ref().is_none_or(|u| &f.unit == u)
                    && filter.from.as_ref().is_none_or(|d| &f.date >= d)
                    && filter.to.as_ref().is_none_or(|d| &f.date <= d)
                    && filter.fillout_id.as_ref().is_none_or(|id| &f.id == id)
            }) {
                for r in f.answers.iter().filter(|r| {
                    r.question_id == q.id
                        && r.context_id
                            .as_ref()
                            .is_none_or(|id| targets.as_ref().is_none_or(|set| set.contains(id)))
                }) {
                    if let Some(i) = r.choice {
                        counts[i] += 1;
                    } else {
                        unanswered += 1;
                    }
                    details.push(json!({"surveyId":s.id,"filloutId":f.id,"respondent":f.respondent,"date":f.date,"unit":f.unit,"completed":f.completed,"contextId":r.context_id,"choice":r.choice,"remark":r.remark}));
                }
            }
        }
        let worst_choice = if q.positive_first {
            counts.iter().rposition(|n| *n > 0)
        } else {
            counts.iter().position(|n| *n > 0)
        };
        if filter.negative_only {
            let negative = if q.most_negative {
                worst_choice.is_some_and(|i| choices[i].tendency == "negative")
            } else {
                counts
                    .iter()
                    .enumerate()
                    .any(|(i, n)| *n > 0 && choices[i].tendency == "negative")
            };
            if !negative {
                continue;
            }
        }
        out.push(ResultRow {
            question_id: q.id.clone(),
            title: q.title.clone(),
            choices,
            counts,
            unanswered,
            worst_choice,
            most_negative: q.most_negative,
            details,
        });
    }
    Ok(out)
}
impl CaseStore {
    pub fn save_audit(&mut self, a: &Audit, expected: Option<&Audit>) -> Result<CaseSnapshot> {
        let current = audits(&self.active.as_ref().ok_or("No case is open")?.db)?
            .into_iter()
            .find(|n| n.id == a.id);
        if current.as_ref() != expected {
            return Err("The audit changed while editing; reload before saving".into());
        }
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        write_bundle(&tx, a)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn delete_audit(&mut self, id: &str, expected: &Audit) -> Result<CaseSnapshot> {
        let current = audits(&self.active.as_ref().ok_or("No case is open")?.db)?
            .into_iter()
            .find(|n| n.id == id);
        if current.as_ref() != Some(expected) {
            return Err("The audit changed before removal".into());
        }
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        clear_children(&tx, id)?;
        if tx
            .execute("DELETE FROM audit WHERE id=?", [id])
            .map_err(err)?
            != 1
        {
            return Err("Audit does not exist".into());
        }
        tx.commit().map_err(err)?;
        self.edited()
    }
    pub fn audit_results(&self, id: &str, filter: &ResultFilter) -> Result<Vec<ResultRow>> {
        let db = &self.active.as_ref().ok_or("No case is open")?.db;
        let audit = audits(db)?
            .into_iter()
            .find(|a| a.id == id)
            .ok_or("Audit does not exist")?;
        results(&audit, filter, db)
    }
}
