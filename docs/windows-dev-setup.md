# Windows Development/Test Setup for openWidGet

This guide prepares a Windows 11 machine for openWidGet Phase 0 spikes:

- Anchor Shortcut lifecycle
- desktop icon grid/positioning
- WebView overlay strategy
- screenshot/video evidence capture

## 0. Recommended access model

Use **Tailscale + RDP + OpenSSH**.

- **RDP** is needed for real desktop/overlay/Tray/z-order verification.
- **SSH** is useful for commands, logs, builds, and scripts.
- Avoid posting passwords/tokens in chat. Prefer SSH keys, Tailscale identity, or manual login.

If Windows is Home edition and cannot host RDP, use Tailscale + OpenSSH plus a separate GUI remote tool such as RustDesk/Parsec/AnyDesk, or have the machine owner capture screenshots manually.

---

## 1. Install Tailscale

Run PowerShell as normal user or Administrator:

```powershell
winget install --id Tailscale.Tailscale -e
```

Then:

1. Open Tailscale.
2. Log in to the same tailnet.
3. Rename the device clearly, e.g. `openwidget-win`.
4. Copy the Tailscale hostname/IP for later.

Check from PowerShell:

```powershell
tailscale status
```

---

## 2. Enable RDP for GUI verification

> Requires Windows Pro/Enterprise/Education as host. Windows Home can connect to RDP but normally cannot host it.

Run PowerShell as Administrator:

```powershell
Set-ItemProperty -Path 'HKLM:\System\CurrentControlSet\Control\Terminal Server' -Name fDenyTSConnections -Value 0
Enable-NetFirewallRule -DisplayGroup "Remote Desktop"
```

Then confirm:

```powershell
Get-ItemProperty -Path 'HKLM:\System\CurrentControlSet\Control\Terminal Server' -Name fDenyTSConnections
```

Expected: `fDenyTSConnections : 0`

Recommended security:

- Use RDP only over Tailscale IP/hostname.
- Keep the Windows account password private.
- Do not expose RDP directly to the public internet.

---

## 3. Enable OpenSSH Server for command execution

Run PowerShell as Administrator:

```powershell
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
Start-Service sshd
Set-Service -Name sshd -StartupType Automatic
New-NetFirewallRule -Name sshd -DisplayName 'OpenSSH Server' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22
```

Verify:

```powershell
Get-Service sshd
ssh localhost
```

If using SSH keys, add the public key to:

```txt
C:\Users\<WindowsUser>\.ssh\authorized_keys
```

Do not paste private keys or passwords into chat.

---

## 4. Install development prerequisites

Run PowerShell as Administrator or normal user where appropriate:

```powershell
winget install --id Git.Git -e
winget install --id GitHub.cli -e
winget install --id Rustlang.Rustup -e
winget install --id OpenJS.NodeJS.LTS -e
winget install --id Microsoft.EdgeWebView2Runtime -e
```

Install Visual Studio Build Tools with C++ workload:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

Restart PowerShell, then verify:

```powershell
git --version
gh --version
rustc --version
cargo --version
node --version
npm --version
```

Tauri can be installed later once the project skeleton exists.

---

## 5. Clone repository

Public clone does not require GitHub auth:

```powershell
mkdir C:\Dev -Force
cd C:\Dev
git clone https://github.com/hoddukzoa12/openWidGet.git
cd openWidGet
git status --short --branch
```

Optional GitHub CLI auth for issue/PR operations:

```powershell
gh auth login
```

---

## 6. Evidence capture folder

Create a local evidence folder:

```powershell
mkdir C:\Dev\openWidGet\qa-artifacts -Force
```

Use it for:

- screenshots
- short videos
- spike logs
- environment notes

Do not capture secrets, tokens, browser password managers, or private chat windows.

---

## 7. Phase 0 readiness checklist

- [ ] Tailscale connected and visible in `tailscale status`.
- [ ] RDP available for GUI verification, or an alternative GUI remote path exists.
- [ ] OpenSSH Server running, if remote command execution is needed.
- [ ] Git, GitHub CLI, Rust, Node, WebView2 Runtime installed.
- [ ] Visual Studio Build Tools C++ workload installed.
- [ ] `hoddukzoa12/openWidGet` cloned.
- [ ] `qa-artifacts/` folder created.
- [ ] Screen capture path tested.

Once this checklist is complete, Phase 0 can start with:

- #2 Anchor Shortcut lifecycle
- #3 desktop icon positioning and grid detection
- #4 desktop overlay window strategy
