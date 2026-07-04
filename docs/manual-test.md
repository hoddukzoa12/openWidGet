# Manual Test Checklist

Use this checklist for checkpoint testing on a real Windows machine.

## Skeleton launch

`npm run build` only verifies the frontend bundle. Tauri desktop launch requires Rust/Cargo.

First check:

```powershell
rustc --version
cargo --version
```

If either command is missing, install Rustup and the Visual Studio C++ build tools, then reopen PowerShell:

```powershell
winget install --id Rustlang.Rustup -e
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

Then run:

```powershell
git clone https://github.com/hoddukzoa12/openWidGet.git
cd openWidGet
npm install
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri:dev
```

Expected:

- OpenWidGet window opens.
- The shell preview displays Clock, Memo, and Project Launcher cards.
- Refresh status button works.
- A tray icon/menu is present if the OS supports the current tray stub.
- Closing/quitting does not leave unexpected generated files in the repo.

Capture:

- PowerShell output
- screenshot of the app window
- note whether tray menu appears
- any WebView2 or Tauri errors

## Release gate

Do not publish `.exe` or public demo claims until Windows smoke evidence exists.
