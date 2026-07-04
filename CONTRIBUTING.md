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

## Pull request expectations

- Keep PRs small and tied to an issue when possible.
- Include the verification command you ran.
- Do not commit secrets, `.env*`, generated installers, or local QA recordings.
- For Windows shell behavior, include a manual Windows test note or mark the PR as needing Windows validation.

## Documentation language policy

- English is the primary OSS surface: `README.md`, `CONTRIBUTING.md`, `ROADMAP.md`, and core docs.
- Korean docs live separately in `README.ko.md` and `docs/ko/`.
- Avoid long bilingual blocks inside one section.

## Good first issues

Good first issues will be added after the widget manifest and sample widget format stabilize.
