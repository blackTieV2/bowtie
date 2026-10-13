use crate::case::{CaseSnapshot, CaseStore};
use crate::editing::Position;
use std::{path::Path, sync::Mutex};
use tauri::State;

type Store<'a> = State<'a, Mutex<CaseStore>>;
#[tauri::command]
pub fn case_save_audit(
    state: Store<'_>,
    audit: crate::audit::Audit,
    expected: Option<crate::audit::Audit>,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.save_audit(&audit, expected.as_ref()))
}
#[tauri::command]
pub fn case_delete_audit(
    state: Store<'_>,
    id: String,
    expected: crate::audit::Audit,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.delete_audit(&id, &expected))
}
#[tauri::command]
pub fn case_audit_results(
    state: Store<'_>,
    id: String,
    filter: crate::audit::ResultFilter,
) -> Result<Vec<crate::audit::ResultRow>, String> {
    with_store(state, |s| s.audit_results(&id, &filter))
}
#[tauri::command]
pub fn case_reorder_lookup(
    state: Store<'_>,
    id: String,
    direction: i64,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.reorder_lookup(&id, direction))
}
#[tauri::command]
pub fn case_replace_preview(
    state: Store<'_>,
    find: String,
    replacement: String,
) -> Result<crate::case_tools::ReplacePreview, String> {
    with_store(state, |s| s.replace_preview(&find, &replacement))
}
#[tauri::command]
pub fn case_replace_apply(
    state: Store<'_>,
    find: String,
    replacement: String,
    signature: String,
    selected: Vec<usize>,
) -> Result<crate::case_tools::ToolResult, String> {
    with_store(state, |s| {
        s.replace_apply(&find, &replacement, &signature, &selected)
    })
}
#[tauri::command]
pub fn case_tool_undo(state: Store<'_>) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.undo_tool())
}
#[tauri::command]
pub fn case_table_export(state: Store<'_>) -> Result<crate::exchange::TableExport, String> {
    with_store(state, |s| s.table_export())
}
#[tauri::command]
pub fn case_read_table(state: Store<'_>, path: String) -> Result<Vec<u8>, String> {
    with_store(state, |s| s.read_table(Path::new(&path)))
}
#[tauri::command]
pub fn case_write_table(
    state: Store<'_>,
    path: String,
    bytes: Vec<u8>,
    signature: String,
) -> Result<(), String> {
    with_store(state, |s| {
        s.write_table(Path::new(&path), &bytes, &signature)
    })
}
#[tauri::command]
pub fn case_import_preview(
    state: Store<'_>,
    text: String,
) -> Result<crate::exchange::ImportPreview, String> {
    with_store(state, |s| s.import_preview(&text))
}
#[tauri::command]
pub fn case_import_apply(
    state: Store<'_>,
    text: String,
    signature: String,
) -> Result<crate::exchange::ImportResult, String> {
    with_store(state, |s| s.import_apply(&text, &signature))
}
#[tauri::command]
pub fn case_compare(state: Store<'_>, path: String) -> Result<crate::compare::Comparison, String> {
    with_store(state, |s| s.compare_case(Path::new(&path)))
}
#[tauri::command]
pub fn case_comparison_report(
    state: Store<'_>,
    path: String,
    destination: String,
    signature: String,
) -> Result<(), String> {
    with_store(state, |s| {
        s.write_comparison(Path::new(&path), Path::new(&destination), &signature)
    })
}
#[tauri::command]
pub fn case_export_context(state: Store<'_>) -> Result<crate::export::ExportContext, String> {
    with_store(state, |s| s.export_context())
}
#[tauri::command]
pub fn case_write_export(
    state: Store<'_>,
    path: String,
    format: String,
    bytes: Vec<u8>,
    signature: String,
) -> Result<(), String> {
    with_store(state, |s| {
        s.write_export(Path::new(&path), &format, &bytes, &signature)
    })
}
fn with_store<T>(
    state: Store<'_>,
    operation: impl FnOnce(&mut CaseStore) -> Result<T, String>,
) -> Result<T, String> {
    let mut store = state
        .lock()
        .map_err(|_| "The case session is unavailable".to_string())?;
    operation(&mut store)
}
#[tauri::command]
pub fn case_get(state: Store<'_>) -> Result<Option<CaseSnapshot>, String> {
    with_store(state, |s| s.snapshot())
}
#[tauri::command]
pub fn case_create(
    state: Store<'_>,
    title: String,
    organization: String,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.create(&title, &organization))
}
#[tauri::command]
pub fn case_open(state: Store<'_>, path: String) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.open(Path::new(&path)))
}
#[tauri::command]
pub fn case_save(state: Store<'_>, path: Option<String>) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.save(path.as_deref().map(Path::new)))
}
#[tauri::command]
pub fn case_close(state: Store<'_>) -> Result<(), String> {
    with_store(state, |s| {
        s.close();
        Ok(())
    })
}
#[tauri::command]
pub fn case_add_group(
    state: Store<'_>,
    parent_id: String,
    title: String,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.add_group(&parent_id, &title))
}
#[tauri::command]
pub fn case_add_diagram(
    state: Store<'_>,
    parent_id: String,
    hazard_title: String,
    top_event_title: String,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| {
        s.add_diagram(&parent_id, &hazard_title, &top_event_title)
    })
}
#[tauri::command]
pub fn case_add_entity(
    state: Store<'_>,
    parent_id: String,
    kind: String,
    title: String,
    description: String,
    properties: serde_json::Value,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| {
        s.add_entity(&parent_id, &kind, &title, &description, &properties)
    })
}
#[tauri::command]
pub fn case_edit_entity(
    state: Store<'_>,
    id: String,
    title: String,
    description: String,
    properties: serde_json::Value,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| {
        s.edit_entity(&id, &title, &description, &properties)
    })
}
#[tauri::command]
pub fn case_delete_entity(state: Store<'_>, id: String) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.delete_entity(&id))
}
#[tauri::command]
pub fn case_set_positions(
    state: Store<'_>,
    diagram_id: String,
    positions: Vec<Position>,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.set_positions(&diagram_id, &positions))
}
#[tauri::command]
pub fn case_link(
    state: Store<'_>,
    source_id: String,
    target_id: String,
    relation: String,
    remove: bool,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.link(&source_id, &target_id, &relation, remove))
}
#[tauri::command]
pub fn case_save_matrix(
    state: Store<'_>,
    matrix: crate::risk::Matrix,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.save_matrix(&matrix))
}
#[tauri::command]
pub fn case_delete_matrix(state: Store<'_>, id: String) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.delete_matrix(&id))
}
#[tauri::command]
pub fn case_set_assessment(
    state: Store<'_>,
    assessment: crate::risk::Assessment,
    remove: bool,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.set_assessment(&assessment, remove))
}
#[tauri::command]
pub fn case_clear_assessments(state: Store<'_>) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.clear_assessments())
}
#[tauri::command]
pub fn case_save_lookup(
    state: Store<'_>,
    lookup: crate::risk::Lookup,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.save_lookup(&lookup))
}
#[tauri::command]
pub fn case_delete_lookup(state: Store<'_>, id: String) -> Result<CaseSnapshot, String> {
    with_store(state, |s| s.delete_lookup(&id))
}
#[tauri::command]
pub fn case_custom_field(
    state: Store<'_>,
    kind: String,
    name: String,
    field_type: String,
    old_name: Option<String>,
    remove: bool,
) -> Result<CaseSnapshot, String> {
    with_store(state, |s| {
        s.custom_field(&kind, &name, &field_type, old_name.as_deref(), remove)
    })
}
