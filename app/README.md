# Risk Case Workspace — Phase 1 scaffold

Temporary neutral working label; final name awaits owner review. Tauri 2, React/TypeScript, Konva and bundled SQLite binding are configured. The UI is an empty shell, not a diagram editor. No case open/save or database command is implemented: D-006 and master-prompt rule 5 require schema/scaffold review before application feature code.

From this directory: `npm ci`, `npm run check`, `npm run build`, `npm run dev`. With Rust and platform prerequisites installed: `npm run tauri dev` or `npm run tauri build -- --no-bundle`. The schema is in `src-tauri/schema/001-initial.sql`; validate from the repository root with `python scripts/check_phase1_schema.py`.

The packaged frontend uses local assets and disallows network connections. Build dependency downloads require a development network/cached dependencies; this is distinct from offline application runtime. No HTTP/updater/opener/activation plugin or filesystem IPC is enabled. Local document references are data only in the schema.

Node dependencies have an npm lockfile. Rust lockfile/native builds must be produced and reviewed on a host with the Rust toolchain; this scaffold does not claim native validation on Windows/macOS/Linux. No icons/branding are copied from another product. Installer packaging is disabled at this review gate.

See `../stages/04-build/output/phase-1-report.md` for exact checks and limits.
