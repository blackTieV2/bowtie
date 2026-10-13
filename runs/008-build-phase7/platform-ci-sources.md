# Platform CI sources

Workflow uses windows-latest, macos-latest and ubuntu-latest with independent matrix failures, frontend build, locked cargo tests, native no-bundle build and distinct binary artifacts. No releases/deployments. Runtime remains offline; build runners fetch pinned lockfile dependencies.

Linux package setup follows [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) and [Tauri GitHub pipeline documentation](https://v2.tauri.app/distribute/pipelines/github/). Artifact upload/matrix behavior follows [GitHub workflow artifacts](https://docs.github.com/en/actions/concepts/workflows-and-actions/workflow-artifacts). Checked 2026-10-13.

The authorized feature branch must be pushed during implementation to trigger CI; final acceptance requires a passing run on the final implementation revision. Early CI evidence does not constitute final Phase 7 acceptance.
