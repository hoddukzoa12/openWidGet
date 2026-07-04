# OpenWidGet

**Open widgets. Get only what your desktop needs.**

[한국어 README](./README.ko.md)

OpenWidGet is an open-source Windows desktop widget runtime for hackable HTML/CSS/JS widgets.

> Status: `v0.1-alpha` skeleton in progress. This is an early public proof-of-work, not a production-ready widget platform yet.

## Concept

Windows desktop widgets often float above the desktop without truly coexisting with the icon grid. openWidGet explores a different model:

```txt
Widget = Live Shortcut Group + Visual Overlay + Data Sources + Actions + Permissions
```

A widget occupies Windows desktop icon-grid space through runtime **Anchor Shortcuts**, then renders a modern HTML/CSS/JS widget overlay above that space.

## Current alpha shell

The first implementation checkpoint adds:

- Tauri/Rust desktop app skeleton
- WebView frontend preview surface
- Rust command bridge via `get_app_status`
- Tray menu stub: Show, Hide Window, Quit
- English-primary docs with separated Korean docs

Run locally:

```bash
npm install
npm run build
npm run tauri:dev
```

Windows manual testing is tracked in [docs/manual-test.md](docs/manual-test.md).

## Planned MVP

- Rust core engine
- Tauri/WebView2 overlay runtime
- HTML/CSS/JS widget packages
- `widget.json` manifest format
- Runtime-only Anchor Shortcut groups
- Windows tray background agent
- Startup auto-restore
- On-demand widget gallery
- App/environment-based widget recommendations
- PR-first community widget registry

## Example widgets

- Calendar / Deadline
- Weather
- GitHub Repo Status
- Project Launcher
- System Monitor
- Market Watch

## Documentation

- [Product & Implementation Plan](docs/openWidGet-product-implementation-plan.md)
- [Architecture](docs/architecture.md)
- [Widget Format Draft](docs/widget-format.md)
- [Manual Test Checklist](docs/manual-test.md)
- [Licensing Policy](docs/licensing.md)
- [한국어 README](README.ko.md)

## Status

`v0.1-alpha` shell implementation has started. The current checkpoint is a Tauri app skeleton with a WebView preview and tray stub; full Windows overlay/runtime behavior is still planned.

## License

openWidGet uses a hybrid open-source licensing strategy:

- **Core/runtime/desktop app:** AGPL-3.0-or-later
- **Widget templates and example starter code:** MIT
- **Community widgets:** Any OSI-approved license declared in `widget.json`

See [docs/licensing.md](docs/licensing.md) for details.
