# OpenWidGet Product-Proof Surface

> Issue: #28 — Design product-proof surface for Windows desktop widget runtime

## Product thesis reminder

OpenWidGet is **not** a generic dark dashboard and not a normal Tauri settings app.

OpenWidGet is a Windows desktop widget runtime:

```text
Widget = Live Shortcut Group + Visual Overlay + Data Sources + Actions + Permissions
```

The first viewport must prove this at a glance: **HTML/CSS/JS widgets live on the Windows desktop icon grid.**

> Note: the dedicated `designer` profile was unavailable in this run because its Codex auth is not currently initialized. This document is a default/PM product-design source-of-truth draft and should be treated as the implementation constraint for #28 until a dedicated designer pass supersedes it.

## First-viewport composition

The alpha shell should open directly into a product proof surface, not a marketing hero.

1. **Top runtime bar**
   - Left: OpenWidGet lockup, version, current mode.
   - Right: product proof label: `Windows desktop widget runtime` and the formula `Live Shortcut Group + Overlay + Actions`.

2. **Main desktop surface**
   - A simulated Windows desktop frame with title line: `Windows desktop · primary monitor`.
   - Visible icon-grid lines and ordinary desktop icons around the edges.
   - A taskbar strip at the bottom with OpenWidGet active.
   - Overlay widgets placed over grid cells, each showing its underlying Anchor Shortcut footprint.

3. **Widget overlays**
   - Clock 2×2.
   - Weather 2×2.
   - Project Launcher 4×2.
   - System Monitor 2×2.
   - Each widget should read as a live desktop object, not a SaaS dashboard card.

4. **Runtime receipt panel**
   - Selected widget name and manifest path.
   - `widget.json` snippet.
   - Permissions list.
   - Actions list.
   - Lifecycle strip: startup → materialize anchors → render overlay → cleanup on quit.

## Component map

- `RuntimeTopbar`
  - Product identity, version, proof formula.
- `DesktopSurface`
  - Copy block + Windows desktop frame.
- `DesktopGrid`
  - Simulated icon grid, desktop icons, taskbar, overlay widgets.
- `RuntimeWidget`
  - Visual overlay card positioned by grid area.
  - Shows size, runtime status, live metric, permissions.
  - Dashed inset and anchor dots represent Anchor Shortcut slots.
- `WidgetReceipt`
  - Manifest, permissions, actions, lifecycle, refresh button.

## Interaction and state requirements

- Clicking a widget selects it and updates the receipt panel.
- Clock widget updates every second.
- `Refresh status` still calls the Tauri command bridge when available.
- The UI must still render in plain browser preview through fallback status.
- Keyboard focus on widgets should be visible.
- `prefers-reduced-motion` should disable non-essential transition timing.

## Visual tokens

```text
Background: #07090d / #0a0f18
Surface:    #0d1118, #141a24, #1b2330
Ink:        #f4f7fb
Muted:      #a6b0bf
Line:       rgba(204, 218, 255, 0.16)
Mint:       #6fffc1
Sky:        #77d4ff
Violet:     #a98cff
Amber:      #ffd166
Danger:     #ff6b7a
```

Typography:

- System UI stack.
- Large compressed headline with negative tracking for product proof copy.
- Monospace only for formula and `widget.json` snippet.

Shape and surface:

- Use grid lines, desktop frame, taskbar, and anchor outlines as the main visual language.
- Rounded overlays are allowed because widgets are overlays, but avoid generic feature-card grids.
- Dashed anchor borders should communicate “reserved desktop icon slots.”

Motion posture:

- Subtle hover/focus elevation only.
- No decorative looping motion.
- No animated gradient hero.

## Copy snippets

Primary headline:

```text
HTML/CSS/JS widgets, pinned to the Windows desktop grid.
```

Support copy:

```text
OpenWidGet reserves desktop icon-grid space with runtime Anchor Shortcuts, then renders live WebView widgets above those slots.
```

Topbar proof label:

```text
Windows desktop widget runtime
Live Shortcut Group + Overlay + Actions
```

Runtime lifecycle:

```text
startup → materialize anchors → render overlay → cleanup on quit
```

## Acceptance criteria for implementation PR

- First viewport visibly communicates **Windows desktop widget runtime**.
- Desktop grid/frame is visible without scrolling on normal desktop viewport.
- At least four widgets are positioned as overlays on the desktop grid.
- Widget selection updates a manifest/actions/permissions receipt.
- The product proof formula appears on the surface.
- No generic SaaS hero/card-grid language dominates the first read.
- `npm run build` passes.
- PR includes screenshot evidence.
- PR body answers: “How does this make OpenWidGet closer to a Windows desktop widget runtime?”

## Anti-goals / anti-slop rules

- Do not lead with “Tauri shell preview.” That is an implementation detail.
- Do not make a generic dashboard with metric cards.
- Do not hide the Windows desktop/icon-grid metaphor.
- Do not overuse glassmorphism, rainbow gradients, or fake analytics.
- Do not imply the real Anchor Shortcut/overlay engine is complete. This is a product-proof shell view.

## Implementation constraints

- Implement within existing Vite/Tauri source files first: `src/main.ts`, `src/styles.css`.
- Keep Rust command bridge compatible; richer backend data can come later.
- Do not introduce heavy UI dependencies for this issue.
- Treat real Anchor Shortcut lifecycle, icon grid detection, and overlay window behavior as follow-up implementation issues (#2, #3, #4).
