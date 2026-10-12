// Schema proposal embedded for review; no case-file commands are exposed yet.
pub const INITIAL_SCHEMA: &str = include_str!("../schema/001-initial.sql");
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Unable to start the local workspace");
}
