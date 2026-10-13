# Barrier Atlas — offline desktop runtime

The owner selected **Barrier Atlas**. The current branch includes case persistence and explorer, an editable Konva diagram, typed properties/risk matrices/actions, diagram/Word-compatible reporting, and Phase 5 case tools. Phase 5 adds validated read-only comparison and diff reports, previewed transactional literal find/replace, original CSV/XLSX exchange and Standard lookup ordering. See [CASE-TABLE-FORMAT.md](CASE-TABLE-FORMAT.md) for the original interchange contract. Human phase-gate approval remains required before Phase 6.

From this directory: `npm ci`, `npm run check`, `npm run build`, `npm run dev`. With Rust and platform prerequisites installed: `npm run tauri dev` or `npm run tauri build -- --no-bundle`. The schema is in `src-tauri/schema/001-initial.sql`; validate from the repository root with `python scripts/check_phase1_schema.py`.

The packaged frontend uses local assets and disallows remote network connections. Build dependency downloads require a development network/cached dependencies; this differs from offline runtime. Native file dialogs select local `.sqlite` files. No HTTP/updater/opener/activation plugin, raw SQL or generic filesystem IPC is exposed. Local document references remain data only.

Node and Rust dependencies are locked. Windows native build results appear in the review report; macOS/Linux remain separate host checks. No icons/branding are copied. Installer packaging remains disabled.

Cases are edited in an in-memory SQLite session. Open reads and validates an original Barrier Atlas schema/format, then copies it into memory. Save creates a validated SQLite file in the target directory and atomically replaces the destination after closing the database handle. Failed operations retain the session/dirty state; Save never overwrites an unrelated SQLite database. Existing `.sqlite` files must match this application's versioned schema and format metadata; this is not a general or legacy SQLite importer.

Every runtime connection enables foreign keys. Newly authored IDs are opaque UUID v4 values; imports preserve original table identities. Diagram/top-event creation, replacement batches and imported batches each use one transaction. Run `cargo test --locked --offline` from `src-tauri` for round-trip, foreign-key, identity, rollback and failure-preservation checks. Unsaved changes prompt before switching/closing cases or closing the native window. Tool undo is limited to the last unchanged unsaved replace/import operation; there is no general history/recovery store.

See `../stages/04-build/output/phase-5-report.md` for exact checks, review status and platform limits.
