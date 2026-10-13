mod audit;
mod case;
mod case_tools;
mod commands;
mod compare;
mod editing;
mod exchange;
mod export;
mod incident;
#[cfg(test)]
mod phase3_tests;
#[cfg(test)]
mod phase6_tests;
#[cfg(test)]
mod phase7_tests;
mod properties;
mod risk;
pub const INITIAL_SCHEMA: &str = include_str!("../schema/001-initial.sql");
pub const RCA_LINK_MIGRATION: &str = include_str!("../schema/002-rca-event-links.sql");
#[cfg(test)]
mod migration_tests;
pub fn run() {
    tauri::Builder::default()
        .manage(std::sync::Mutex::new(case::CaseStore::default()))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::case_get,
            commands::case_save_incident,
            commands::case_delete_incident,
            commands::case_save_audit,
            commands::case_delete_audit,
            commands::case_audit_results,
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
            commands::case_save_matrix,
            commands::case_delete_matrix,
            commands::case_set_assessment,
            commands::case_clear_assessments,
            commands::case_save_lookup,
            commands::case_delete_lookup,
            commands::case_reorder_lookup,
            commands::case_custom_field,
            commands::case_export_context,
            commands::case_write_export,
            commands::case_compare,
            commands::case_comparison_report,
            commands::case_replace_preview,
            commands::case_replace_apply,
            commands::case_tool_undo,
            commands::case_table_export,
            commands::case_read_table,
            commands::case_write_table,
            commands::case_import_preview,
            commands::case_import_apply,
        ])
        .run(tauri::generate_context!())
        .expect("Unable to start Barrier Atlas");
}
