# Spike: Windows Anchor Shortcut lifecycle

Issue: #2

## Goal

Prove the Phase 0 OpenWidGet thesis that a widget footprint can be projected onto the real Windows Desktop as runtime-owned Anchor Shortcuts and safely cleaned up by the app.

The first runnable proof is intentionally narrow:

- one sample `clock` widget,
- one 2×2 Live Shortcut Group,
- four `.lnk` Anchor Shortcuts,
- transparent icon materialized by the runtime,
- cleanup on normal process exit,
- stale artifact detection on the next startup.

## Runtime behavior

On Windows app startup, OpenWidGet now:

1. resolves the Desktop folder using `%USERPROFILE%\Desktop`, with common OneDrive Desktop fallbacks;
2. writes a transparent Anchor Shortcut icon into `%LOCALAPPDATA%\OpenWidGet\anchor-shortcuts\openwidget-anchor-transparent.ico`;
3. scans the Desktop for prior `OpenWidGet Anchor - *.lnk` files whose session does not match the current app session;
4. deletes stale OpenWidGet-owned Anchor Shortcuts;
5. creates four current-session `.lnk` files for the 2×2 `clock` widget.

On normal app exit, OpenWidGet deletes the current session's four Anchor Shortcuts.

The `.lnk` files are named like:

```text
OpenWidGet Anchor - clock - <session> - r1c1.lnk
OpenWidGet Anchor - clock - <session> - r1c2.lnk
OpenWidGet Anchor - clock - <session> - r2c1.lnk
OpenWidGet Anchor - clock - <session> - r2c2.lnk
```

Each shortcut points at the current OpenWidGet executable with safe placeholder arguments:

```text
--open-anchor --widget-id clock --anchor-id <anchor-id> --session-id <session>
```

## UI receipt

The product-proof surface now calls `get_anchor_lifecycle_status` and shows:

- platform,
- runtime session id,
- anchors planned/created,
- stale anchors detected/deleted,
- Desktop path,
- any fallback/error message.

On non-Windows development machines this stays in preview mode and does not touch the Desktop.

## Verification commands

Run locally on macOS/Linux for compile-time and pure lifecycle checks:

```bash
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml anchor_shortcuts
cargo check --manifest-path src-tauri/Cargo.toml
```

Run on the Windows checkpoint machine for real Desktop evidence:

```powershell
cd C:\Users\jinuk\openWidGet
git pull
npm install
npm run build
cargo test --manifest-path src-tauri/Cargo.toml anchor_shortcuts
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri:dev
```

While the app is running, confirm the Desktop has four transparent OpenWidGet Anchor `.lnk` files. After quitting the app from the tray/menu or closing the app normally, confirm those four files disappear.

## Evidence to attach to issue #2 / PR

- PowerShell log for the Windows commands above.
- Screenshot/video while the app is running showing four `OpenWidGet Anchor - clock - ...` files on Desktop.
- Screenshot/log after normal exit showing those files are gone.
- Optional restart test: manually leave one old `OpenWidGet Anchor - clock - old - r1c1.lnk`, start the app, and confirm the UI receipt reports stale cleanup.

## Known risks / follow-ups

- Desktop path resolution covers normal and common OneDrive Desktop paths, but a future hardening pass should use the Windows Known Folder API for non-standard Desktop redirects.
- PowerShell/WScript COM is used for this spike to avoid extra native dependencies. If antivirus/user-trust concerns appear, replace it with native Rust COM shortcut creation.
- This issue proves lifecycle only. Exact icon-grid positioning is still #3, and overlay alignment/z-order is still #4.
