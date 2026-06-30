# openWidGet

**Open widgets. Get only what your desktop needs.**

openWidGet is an open-source Windows desktop widget platform that turns static desktop shortcut space into live, icon-backed widgets.

## Concept

Windows desktop widgets often float above the desktop without truly coexisting with the icon grid. openWidGet explores a different model:

```txt
Widget = Live Shortcut Group + Visual Overlay + Data Sources + Actions + Permissions
```

A widget occupies Windows desktop icon-grid space through runtime **Anchor Shortcuts**, then renders a modern HTML/CSS/JS widget overlay above that space.

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

## Status

Planning/spec phase. Implementation has not started yet.

## License

MIT
