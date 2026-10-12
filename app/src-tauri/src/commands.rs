use crate::case::{CaseSnapshot, CaseStore};
use crate::editing::Position;
use std::{path::Path, sync::Mutex};
use tauri::State;

type Store<'a> = State<'a, Mutex<CaseStore>>;
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
