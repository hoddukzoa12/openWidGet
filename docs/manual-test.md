# Manual Test Checklist

Use this checklist for checkpoint testing on a real Windows machine.

## Skeleton launch

```powershell
git clone https://github.com/hoddukzoa12/openWidGet.git
cd openWidGet
npm install
npm run build
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
