# Spike: Desktop icon grid positioning

Issue: #3

## Goal

Prove the next Phase 0 question after Anchor Shortcut lifecycle: OpenWidGet can map widget footprints to Windows desktop icon-grid geometry and make an explicit decision about direct positioning versus a safe reconciliation fallback.

This spike intentionally does **not** move shortcuts on every drag frame. The product rule is:

> Move the WebView/overlay preview during drag, then commit Anchor Shortcut group positions only once on drop.

## Runtime behavior

OpenWidGet now exposes a `get_desktop_grid_status` Tauri command.

On Windows it attempts to probe:

1. primary monitor bounds and work area from `System.Windows.Forms.Screen.PrimaryScreen`;
2. DPI from `System.Drawing.Graphics.FromHwnd(IntPtr.Zero)`;
3. icon cell spacing from `HKCU:\Control Panel\Desktop\WindowMetrics` (`IconSpacing` and `IconVerticalSpacing`).

The runtime converts that measurement into deterministic widget footprint plans:

- `clock` — 2×2 Live Shortcut Group, 4 Anchor Shortcut slots;
- `launcher` — 4×2 Live Shortcut Group, 8 Anchor Shortcut slots.

It also attempts a Windows Desktop ListView reconciliation probe for existing `OpenWidGet Anchor` icons:

1. locate the desktop ListView under `Progman` / `WorkerW` (`SHELLDLL_DefView` → `SysListView32`);
2. open the Explorer process and read ListView item text/positions with remote memory plus `LVM_GETITEMTEXTW` / `LVM_GETITEMPOSITION`;
3. filter observed desktop icons whose names begin with `OpenWidGet Anchor`;
4. parse the anchor widget id and slot suffix (`clock` + `r1c1`, `launcher` + `r1c4`, ...);
5. expose observed-vs-target deltas in the grid receipt.

Each plan includes:

- grid start row/column;
- full widget rectangle in desktop work-area pixels;
- one rectangle per Anchor Shortcut slot (`r1c1`, `r1c2`, ...).

On non-Windows development machines the command returns deterministic preview geometry instead of touching the desktop.

## Positioning decision

Current decision: **reconcile implemented, direct mutation still gated.**

This PR implements read-only reconciliation against the real desktop ListView when running on Windows. It does not yet mutate Explorer icon positions. Direct desktop icon mutation remains gated because it must be proven against Explorer auto-arrange, icon-size, and DPI settings on a real Windows desktop.

Implemented Windows API path:

1. locate the desktop ListView under `Progman` / `WorkerW` (`SHELLDLL_DefView` → `SysListView32`);
2. read current OpenWidGet Anchor icon labels and positions with `LVM_GETITEMTEXTW` and `LVM_GETITEMPOSITION`;
3. compare observed anchor positions against computed target slot rectangles;
4. report observed-vs-target deltas in the runtime receipt.

Deferred direct-positioning path:

1. after drop only, batch final positions with `LVM_SETITEMPOSITION` if the Windows smoke test proves it survives Explorer settings;
2. never send per-mousemove shortcut repositioning messages.

Fallback if direct positioning is unreliable:

- keep Anchor Shortcuts materialized and identifiable;
- use the computed grid receipt to align the WebView overlay;
- record the intended Anchor Shortcut group rectangle in config;
- re-probe/reconcile when DPI, icon size, work area, or Explorer auto-arrange behavior changes.

## Limitations

- Primary monitor only for this spike.
- Auto-arrange icons can override direct shortcut positioning.
- Desktop icon size/spacing/DPI changes require a fresh grid probe.
- `LVM_SETITEMPOSITION` proof is deferred until the next Windows manual evidence pass.
- Multi-monitor and per-monitor DPI are out of scope until #4/#7 need them.

## Verification commands

Local compile/test verification:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
```

Windows smoke checklist:

```powershell
cd C:\Users\jinuk\openWidGet
git pull
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri:dev
```

While the app is running:

1. Confirm the receipt card shows `Desktop grid positioning`.
2. Confirm `cell`, `work area`, `dpi`, and `positioning` fields are populated.
3. Confirm plans show at least:
   - `clock` as 2×2 with 4 anchors;
   - `launcher` as 4×2 with 8 anchors.
4. Confirm `reconcile` shows `windows-listview-probe` when the desktop ListView probe succeeds, with observed/matched counts for visible OpenWidGet Anchor icons.
5. Change Windows desktop icon size or DPI only if safe, restart OpenWidGet, and confirm the grid receipt updates or falls back with a warning.
6. Keep Explorer auto-arrange behavior noted in the evidence; do not claim direct mutation until an `LVM_SETITEMPOSITION` smoke proves it.

## Acceptance mapping

- **Compute grid rectangles for 2×2 and 4×2 widgets:** implemented by `desktop_grid.rs` and covered by unit tests.
- **Position or reconcile Anchor Shortcut group positions:** current mode is `reconcile-only`; the app computes the target group/per-anchor slot rectangles and, on Windows, probes the desktop ListView to compare observed OpenWidGet Anchor icon positions against those targets.
- **Fallback if direct positioning is unreliable:** explicit fallback is to align overlay from computed grid receipt and defer shortcut movement to a drop-time batch commit proof.
- **Document Windows API path and limitations:** this document records the `Progman`/`WorkerW`/ListView path, `LVM_GETITEMPOSITION`, `LVM_SETITEMPOSITION`, and limitations.

## Follow-up

- #4 should align the transparent/frameless WebView overlay above the computed 2×2 Anchor Shortcut region.
- A later Windows-only hardening PR can implement and smoke-test the actual `LVM_SETITEMPOSITION` batch commit once Explorer auto-arrange behavior is documented.
