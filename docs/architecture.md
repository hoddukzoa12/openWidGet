# OpenWidGet Architecture

OpenWidGet is planned as a Tauri/Rust desktop runtime with a WebView-powered widget surface.

```txt
OpenWidGet
├─ Tauri shell
│  ├─ app lifecycle
│  ├─ tray menu
│  ├─ settings/config paths
│  └─ future autostart hooks
├─ WebView frontend
│  ├─ widget preview surface
│  ├─ future overlay/edit mode
│  └─ widget action bridge
└─ Widgets
   ├─ widget.json
   ├─ index.html
   ├─ style.css
   └─ widget.js
```

## v0.1-alpha scope

The first alpha does not claim the full desktop overlay/runtime yet. It proves:

- a runnable Tauri app shell;
- a frontend preview surface;
- a Rust command bridge;
- a tray menu stub;
- a path toward Windows packaging and manual smoke evidence.

## Later architecture gates

- Anchor Shortcut lifecycle
- desktop grid detection
- floating overlay window strategy
- widget manifest validator
- permissions and action bridge
