# Barrier Atlas — Phase 1 runtime

The owner approved the schema/scaffold and selected **Barrier Atlas**. Remaining Phase 1 scope adds native case create/open/save/save-as/close and an expandable tree reading SQLite through Tauri IPC. Group creation and transactional hazard/diagram/top-event creation populate the tree. No diagram canvas or Phase 2 interactions are implemented; Konva remains a declared future dependency.

From this directory: `npm ci`, `npm run check`, `npm run build`, `npm run dev`. With Rust and platform prerequisites installed: `npm run tauri dev` or `npm run tauri build -- --no-bundle`. The schema is in `src-tauri/schema/001-initial.sql`; validate from the repository root with `python scripts/check_phase1_schema.py`.

The packaged frontend uses local assets and disallows remote network connections. Build dependency downloads require a development network/cached dependencies; this differs from offline runtime. Native file dialogs select local `.sqlite` files. No HTTP/updater/opener/activation plugin, raw SQL or generic filesystem IPC is exposed. Local document references remain data only.

Node and Rust dependencies are locked. Windows native build results appear in the review report; macOS/Linux remain separate host checks. No icons/branding are copied. Installer packaging remains disabled.

Cases are edited in an in-memory SQLite session. Open reads and validates an original Barrier Atlas schema/format, then copies it into memory. Save creates a validated SQLite file in the target directory and atomically replaces the destination after closing the database handle. Failed operations retain the session/dirty state; Save never overwrites an unrelated SQLite database. Existing `.sqlite` files must match this application's versioned schema and format metadata; this is not a general or legacy SQLite importer.

Every runtime connection enables foreign keys. IDs are opaque UUID v4 values. Diagram/top-event creation is one transaction. Run `cargo test --lib --release` from `src-tauri` for round-trip, foreign-key, identity, rollback and failure-preservation checks. Unsaved changes prompt before switching/closing cases or closing the native window. Ten-minute recovery/undo/other future features are not part of this continuation.

See `../stages/04-build/output/phase-1-report.md` for exact checks and limits.
