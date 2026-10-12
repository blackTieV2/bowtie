mod case;
mod commands;
mod editing;
pub const INITIAL_SCHEMA: &str = include_str!("../schema/001-initial.sql");
pub fn run() {
    tauri::Builder::default()
        .manage(std::sync::Mutex::new(case::CaseStore::default()))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::case_get,
            commands::case_create,
            commands::case_open,
            commands::case_save,
            commands::case_close,
            commands::case_add_group,
            commands::case_add_diagram,
            commands::case_add_entity,
            commands::case_edit_entity,
            commands::case_delete_entity,
            commands::case_set_positions,
            commands::case_link,
        ])
        .run(tauri::generate_context!())
        .expect("Unable to start Barrier Atlas");
}
