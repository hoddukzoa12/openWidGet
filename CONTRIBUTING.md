# Contributing to OpenWidGet

OpenWidGet is an early alpha experiment for hackable Windows desktop widgets.

## Current status

The project is not production-ready yet. The first public goal is a `v0.1-alpha` runtime shell with a small set of bundled widget previews, Windows smoke evidence, and a clear path for future widget contributions.

## Development setup

Prerequisites:

- Node.js 22+
- npm 11+
- Rust stable
- Tauri prerequisites for your OS
- On Windows: Microsoft Edge WebView2 Runtime

Install and run:

```bash
npm install
npm run build
npm run tauri:dev
```

## Issue-driven PR workflow

OpenWidGet uses an **issue-per-PR workflow** from `v0.1-alpha` onward.

1. Start with a GitHub Issue for every non-trivial change.
2. Pick one primary issue per branch/PR.
3. Create a scoped branch named after the issue, for example:
   - `issue-29-issue-driven-pr-workflow`
   - `issue-28-alpha-shell-visual-direction`
   - `fix-20-windows-smoke-blocker`
4. Keep the PR narrow. If the work expands, split follow-up issues instead of growing the PR.
5. Reference the issue in the PR body using `Closes #<issue>` or `Refs #<issue>`.
6. Include real verification evidence before requesting review.
7. Merge only after review/PM acceptance for the issue scope.

Direct commits to `main` are reserved for emergency/admin-only changes. Normal project work should go through branch + PR, including documentation and workflow changes.

## Pull request expectations

- Link exactly one primary issue unless the PR is intentionally a small dependency/update batch.
- State what is in scope and what is out of scope.
- Include the verification commands you ran and their result.
- Include screenshots or manual Windows notes for user-facing desktop behavior.
- Call out remaining risks honestly.
- Do not commit secrets, `.env*`, generated installers, or local QA recordings.
- For Windows shell behavior, include a manual Windows test note or mark the PR as needing Windows validation.

See [docs/development-workflow.md](docs/development-workflow.md) for the full issue -> branch -> PR -> review flow.

## Documentation language policy

- English is the primary OSS surface: `README.md`, `CONTRIBUTING.md`, `ROADMAP.md`, and core docs.
- Korean docs live separately in `README.ko.md` and `docs/ko/`.
- Avoid long bilingual blocks inside one section.

## Good first issues

Good first issues will be added after the widget manifest and sample widget format stabilize.
