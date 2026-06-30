# openWidGet Product & Implementation Plan

> **For Hermes:** This is a planning/spec artifact. Do not start implementation from this document unless Jinu explicitly asks to build/implement/edit/debug/execute.

**Created:** 2026-06-30 17:19 KST  
**Project:** `openWidGet`  
**Meaning:** `Open Widget` + `Get` — 필요한 위젯만 발견하고 가져오는 오픈소스 위젯 플랫폼  
**Contest target:** 2026 제20회 오픈소스 개발자대회 자유과제 후보  
**Primary platform:** Windows 11 desktop  
**Core stack:** Rust core + Tauri/WebView2 + HTML/CSS/JS widgets  

---

## 1. One-line pitch

**openWidGet은 Windows 바탕화면의 정적인 바로가기 아이콘 공간을 `Live Shortcut Group`으로 확장해, 실시간 정보와 앱/사이트 실행 액션을 제공하는 오픈소스 데스크탑 위젯 플랫폼이다.**

짧은 영어 태그라인:

> **Open widgets. Get only what your desktop needs.**

---

## 2. Product thesis

Windows에는 macOS/iOS처럼 바탕화면에 자연스럽게 배치되는 위젯 경험이 부족하다. 기존 대안은 있지만 각각 한계가 있다.

- **BeWidgets / Widget Launcher**: 사용자는 쉽지만, 오픈소스 PR 기반 위젯 생태계가 약함.
- **Rainmeter / JaxCore**: 강력하지만 `.ini`, Measure/Meter, 스킨 구조가 old-school이고 진입장벽이 높음.
- **XWidget / 8GadgetPack**: 오래된 위젯/가젯 생태계 느낌이 강함.
- **작은 Electron/Tauri 위젯 프로젝트들**: 실험적이지만 플랫폼/레지스트리/기여 흐름이 약함.

openWidGet의 차별점은 단순히 위젯을 띄우는 게 아니라:

1. **실제 Windows 바탕화면 아이콘 그리드를 점유한다.**
2. **위젯은 여러 개의 Anchor Shortcut으로 구성된 Live Shortcut Group이다.**
3. **시각 UI는 HTML/CSS/JS로 쉽고 예쁘게 만든다.**
4. **위젯은 필요한 것만 설치한다.**
5. **사용자의 설치 앱/개발환경을 감지해 위젯을 추천한다.**
6. **커뮤니티는 PR로 위젯을 추가한다.**

---

## 3. Core concept

### 3.1 Widget definition

openWidGet에서 위젯은 단순 overlay 카드가 아니다.

```txt
Widget = Live Shortcut Group + Visual Overlay + Data Sources + Actions + Permissions
```

구성:

- **Live Shortcut Group**: 실제 Windows 바탕화면의 투명 Anchor Shortcut 묶음.
- **Visual Overlay**: Anchor Shortcut 영역 위에 렌더링되는 WebView 기반 위젯 UI.
- **Data Sources**: 날씨, GitHub, 시스템 상태, 로컬 서버 등 실시간 정보.
- **Actions**: 클릭 시 앱/사이트/폴더/프로젝트 실행.
- **Permissions**: 네트워크, 시스템 정보, 파일 접근, 명령 실행 권한 선언.

### 3.2 Example layout

사용자 이미지 예시처럼:

```txt
[ Calendar 2x2 ] [ Weather 2x2 ]

[          Market Watch 4x5          ]
```

각 위젯은 픽셀 좌표가 아니라 Windows 바탕화면 아이콘 그리드 단위로 배치한다.

```json
{
  "widgetId": "weather",
  "monitor": "primary",
  "grid": { "x": 2, "y": 0, "w": 2, "h": 2 }
}
```

---

## 4. Non-goals

MVP에서 하지 않을 것:

- Windows 공식 위젯 보드(`Win + W`)용 provider 구현.
- Explorer shell 자체를 교체하거나 패치.
- Rainmeter `.ini`/Measure/Meter 호환성.
- 모든 위젯을 한 번에 번들 설치.
- 임의 JS에 무제한 파일/명령 실행 권한 제공.
- OAuth 토큰을 HTML/JS 위젯에 직접 노출.
- 다중 모니터 완전 대응을 Phase 0부터 보장.
- 앱 종료 후 투명 shortcut을 영구적으로 남기는 동작.

---

## 5. Architecture

```txt
openWidGet
├─ Rust Core
│  ├─ TrayApp
│  ├─ AutoStartManager
│  ├─ DesktopGridEngine
│  ├─ GhostShortcutManager
│  ├─ LayoutManager
│  ├─ WidgetManager
│  ├─ WidgetRegistryManager
│  ├─ AppDetector
│  ├─ DataScheduler
│  ├─ PermissionManager
│  ├─ ActionRunner
│  ├─ AuthManager
│  └─ RecoveryManager
│
├─ WebView Overlay Runtime
│  ├─ WidgetRenderer
│  ├─ EditModeUI
│  ├─ DragSnapController
│  ├─ GridPreview
│  └─ JS Bridge: window.openWidGet
│
├─ Widget Packages
│  ├─ widget.json
│  ├─ index.html
│  ├─ style.css
│  ├─ widget.js
│  ├─ preview.png
│  └─ README.md
│
└─ Community Registry
   ├─ registry.json
   ├─ widgets/*
   └─ GitHub Actions validation
```

---

## 6. Component responsibilities

### 6.1 Rust Core

Rust Core는 OS와 보안 경계에 가까운 일을 담당한다.

- Windows tray 상주.
- Windows 시작 시 자동실행 등록/해제.
- 현재 모니터, work area, DPI, 아이콘 그리드 계산.
- Anchor Shortcut 생성/삭제/위치 지정.
- 위젯 layout/config 저장.
- 위젯 설치/삭제/활성화/비활성화.
- 데이터 수집 스케줄링.
- 권한 검사.
- 앱/URL/폴더 실행.
- OAuth/token 보안 저장.
- crash 후 stale shortcut 복구.

### 6.2 WebView Overlay Runtime

WebView Runtime은 사용자가 보는 위젯 UI와 편집 UX를 담당한다.

- 위젯 HTML/CSS/JS 렌더링.
- 편집 모드에서 grid preview 표시.
- 위젯 drag/snap.
- lock mode에서 일반 바탕화면처럼 표시.
- 위젯 클릭/버튼 이벤트를 Rust Core ActionRunner로 전달.
- Rust Core가 보낸 실시간 데이터 event를 위젯에 전달.

### 6.3 Widget Package

위젯은 폴더 단위로 배포한다.

```txt
widgets/
  github-status/
    widget.json
    index.html
    style.css
    widget.js
    preview.png
    README.md
```

위젯은 token이나 민감 권한을 직접 다루지 않고, `window.openWidGet` API를 통해 제한된 기능만 호출한다.

---

## 7. Ghost shortcut lifecycle

용어는 사용자 신뢰를 위해 `Anchor Shortcut` 또는 `Live Shortcut Group`을 우선 사용한다. 내부적으로는 ghost shortcut이라고 부를 수 있다.

### 7.1 Normal startup

```txt
Windows login
→ openWidGet auto-start
→ layout/config 로드
→ 필요한 Anchor Shortcut group 생성
→ overlay window 생성
→ 위젯 UI 렌더링
→ DataScheduler 시작
```

### 7.2 Normal shutdown

```txt
User quits openWidGet
→ DataScheduler stop
→ overlay 제거
→ Anchor Shortcut 전부 삭제
→ layout/config 유지
→ process exit
```

중요 정책:

> 앱 종료 시 Anchor Shortcut은 반드시 삭제한다. 영구 상태는 config에 저장하고, shortcut은 실행 중에만 materialize되는 runtime desktop projection이다.

### 7.3 Hide widgets

```txt
Hide Widgets
→ overlay 제거
→ Anchor Shortcut 삭제
→ layout/config 유지
```

### 7.4 Show widgets

```txt
Show Widgets
→ config 기반 Anchor Shortcut 재생성
→ overlay 복원
→ data refresh 재개
```

### 7.5 Delete widget

```txt
Delete Widget
→ 해당 overlay 제거
→ 해당 Anchor Shortcut group 삭제
→ layout/config에서 해당 widget 제거
→ settings/secrets 선택 삭제
```

### 7.6 Crash recovery

앱이 crash되면 shortcut이 남을 수 있다. 다음 실행 시:

```txt
openWidGet startup
→ Desktop에서 stale Anchor Shortcut 검색
→ sessionId/config 비교
→ 필요 없는 stale shortcut 삭제
→ 필요한 shortcut은 재생성/재배치
```

---

## 8. Layout model

위젯은 픽셀 좌표가 아니라 grid 단위로 저장한다.

```ts
type WidgetLayout = {
  instanceId: string
  widgetId: string
  monitorId: string
  grid: { x: number; y: number; w: number; h: number }
}
```

프리셋:

```txt
small   = 2×2
wide    = 4×2
medium  = 4×3
large   = 4×4
tall    = 2×4
xlarge  = 6×4
```

MVP에서는 arbitrary resize를 하지 않고 preset size만 지원한다.

---

## 9. Movement and performance model

중요 성능 원칙:

> 드래그 중에는 실제 Anchor Shortcut을 움직이지 않는다. Overlay/preview만 움직이고, drop 시점에만 shortcut 위치를 batch commit한다.

### 9.1 Edit mode drag flow

```txt
User starts dragging widget
→ overlay clone/preview moves smoothly
→ grid highlight updates
→ collision warning 표시
→ user drops widget
→ final grid position 계산
→ Anchor Shortcut group 위치를 한 번에 commit
→ overlay를 committed rectangle에 snap
```

### 9.2 Why

4×4 위젯은 Anchor Shortcut 16개, 4×5 위젯은 20개를 가진다. 이를 마우스 이동마다 Explorer에 반영하면 lag 위험이 크다.

목표:

- 일반 모드 idle CPU: 0~1%
- 편집 드래그: overlay-only로 부드럽게
- drop commit: 30개 내외 shortcut은 짧은 순간에 처리
- WebView는 가능하면 monitor당 1개

---

## 10. Widget manifest schema draft

```json
{
  "id": "github-status",
  "name": "GitHub Status",
  "version": "0.1.0",
  "author": "openWidGet Team",
  "license": "MIT",
  "description": "Shows public GitHub repo status on the desktop.",
  "category": "developer",
  "entry": "index.html",
  "size": {
    "default": [4, 2],
    "min": [2, 2],
    "max": [4, 4]
  },
  "permissions": {
    "network": ["api.github.com"],
    "systemInfo": false,
    "filesystem": false,
    "commands": false,
    "auth": []
  },
  "settings": [
    {
      "key": "repo",
      "type": "string",
      "label": "Repository",
      "placeholder": "owner/repo",
      "required": true
    },
    {
      "key": "refreshInterval",
      "type": "number",
      "label": "Refresh interval seconds",
      "default": 300
    }
  ],
  "actions": {
    "click": {
      "type": "url",
      "target": "https://github.com/{{settings.repo}}"
    }
  },
  "detect": {
    "apps": ["git", "vscode"],
    "files": [".git"]
  }
}
```

---

## 11. Widget JavaScript API draft

위젯은 직접 OS/secret에 접근하지 않고 `window.openWidGet` bridge만 사용한다.

```js
const settings = await openWidGet.settings.get()

const repo = await openWidGet.http.get(
  `https://api.github.com/repos/${settings.repo}`
)

openWidGet.ui.renderStatus({
  title: repo.full_name,
  badge: `${repo.stargazers_count} stars`,
  subtitle: `${repo.open_issues_count} open issues`
})
```

클릭 action:

```js
openWidGet.actions.onClick(async () => {
  await openWidGet.open.url(`https://github.com/${settings.repo}`)
})
```

OAuth는 위젯 JS가 직접 token을 다루지 않는다.

```js
await openWidGet.auth.connect("github")
const prs = await openWidGet.providers.github.request("/repos/owner/repo/pulls")
```

---

## 12. Data Scheduler

Rust Core가 실시간 데이터 수집을 중앙 관리한다.

예상 interval:

```txt
Clock: 1s
System monitor: 2~5s
Local server health: 5~30s
Market watch: 15~60s
GitHub public repo: 3~5m
Weather: 15~30m
Calendar/deadline: 1~5m
```

필수 기능:

- 위젯별 refresh interval.
- 같은 요청 dedupe.
- cache + stale data 표시.
- retry/backoff.
- offline 표시.
- 배터리 모드에서 refresh interval 완화.
- 위젯이 hidden일 때 update pause.

---

## 13. Widget installation strategy

모든 위젯을 한 번에 설치하지 않는다.

상태 모델:

```txt
available: registry에 있음
installed: 다운로드됨
enabled: 사용자가 배치함
running: 현재 overlay에서 실행 중
paused: 비활성화됨
```

### 13.1 Built-in minimum

초기 번들에는 최소 위젯만 포함한다.

- Clock
- Calendar/Deadline
- App Launcher

### 13.2 On-demand gallery

Widget Gallery에서 필요한 것만 설치한다.

- Developer: GitHub, VS Code Project, Localhost Server, Docker Status
- Productivity: Markdown Todo, Calendar, Notion Shortcut
- System: CPU/RAM, Battery, Network
- Finance: Market Watch, Crypto Ticker, FX Rate
- School: LMS Deadline, Timetable

### 13.3 App detection recommendations

첫 실행 또는 설정에서 환경 감지 후 추천한다.

```txt
VS Code detected → Project Launcher Widget 추천
Git detected → Git Repo Widget 추천
Docker detected → Docker Status Widget 추천
Chrome/Edge detected → Web Shortcut Widget 추천
OneDrive Desktop detected → Anchor Shortcut sync warning 표시
```

자동 설치하지 않는다. 사용자가 승인해야 설치한다.

---

## 14. OAuth/security model

MVP에서는 OAuth를 깊게 넣지 않고 public data/API key/manual config 위주로 간다.

v0.2 이후:

```txt
Widget UI
→ openWidGet.auth.connect(provider)
→ Rust Core AuthManager
→ system browser + PKCE/device flow
→ callback 수신
→ token exchange
→ Windows Credential Manager / DPAPI 저장
→ 위젯에는 token이 아니라 데이터만 전달
```

원칙:

- HTML/JS 위젯에 OAuth token 직접 노출 금지.
- client secret이 필요한 provider는 MVP에서 피한다.
- command execution 권한은 기본 금지.
- community widget은 manifest permission + CI scan 필수.

---

## 14.5 Licensing model

openWidGet은 플랫폼 보호와 위젯 생태계 확장성을 동시에 잡기 위해 hybrid licensing을 사용한다.

```txt
Core/runtime/desktop app: AGPL-3.0-or-later
Widget templates/example starter code: MIT
Community widgets: OSI-approved license declared in widget.json
```

정책:

- Rust core, Tauri/WebView runtime, Anchor Shortcut manager, registry manager, permission/auth/data scheduler는 AGPL-3.0-or-later.
- 위젯 템플릿과 예제 starter code는 MIT로 제공하여 개발자가 부담 없이 복사/수정할 수 있게 한다.
- 공식 community registry에 들어오는 위젯은 `widget.json`에 SPDX license를 반드시 선언해야 한다.
- 비상업 전용, 학술 전용, source-available, custom restrictive license는 공식 registry 기본 허용 대상이 아니다.
- third-party library/framework/model/asset은 출처와 라이선스를 문서화한다.

이 정책은 오픈소스 개발자대회의 OSI 인증 라이선스 요구사항을 만족하면서, openWidGet core 개선 사항이 커뮤니티에 환원되도록 한다.

---

## 15. Tray and auto-start UX

openWidGet은 시스템 트레이에 상주한다.

트레이 메뉴 초안:

```txt
openWidGet
────────────────
위젯 보이기 / 숨기기
위젯 편집
위젯 추가
위젯 갤러리
데이터 새로고침
────────────────
상태 검사 / 복구
설정
종료
```

자동실행 정책:

- 첫 실행 온보딩에서 자동실행을 권장한다.
- 사용자가 동의하면 Windows 시작프로그램에 등록한다.
- 설정에서 언제든 끌 수 있다.
- 앱 종료 시 Anchor Shortcut은 삭제된다.
- 다음 부팅/실행 시 config 기반으로 복원된다.

온보딩 문구:

```txt
Windows 시작 시 openWidGet을 자동 실행할까요?
바탕화면 위젯과 Live Shortcut Group을 자동으로 복원합니다.

[자동 실행 켜기] [나중에]
```

---

## 16. MVP widgets

### 16.1 Calendar / Deadline

- Size: 2×2 또는 4×2
- Data: local date, manual deadlines, optional `.ics`
- Action: calendar/contest/LMS URL open
- Value: 학생/해커톤 데모에 좋음

### 16.2 Weather

- Size: 2×2
- Data: public weather API, e.g. Open-Meteo
- Action: forecast page open
- Value: macOS/iOS 위젯 느낌을 빠르게 보여줌

### 16.3 GitHub Repo Status

- Size: 4×2
- Data: public GitHub REST API
- Shows: stars, issues, PR count, last push, Actions status if possible
- Action: repo/PR/Actions open
- Value: 오픈소스 대회와 잘 맞음

### 16.4 Project Launcher

- Size: 2×2 또는 4×2
- Data: local folder path, optional git branch/status
- Action: VS Code/folder open
- Value: “정적인 바로가기를 live shortcut으로 바꾼다”를 가장 잘 보여줌

### 16.5 System Monitor

- Size: 2×2 또는 4×2
- Data: CPU/RAM/battery/network from Rust core
- Action: Task Manager/Settings open
- Value: 실시간 local data 증명

### 16.6 Optional Market Watch

- Size: 4×5
- Data: public finance/crypto APIs or user API key
- Action: TradingView/configured source open
- Value: 사용자가 보낸 예시 이미지와 유사한 데모 가능
- Risk: 금융 데이터 API 라이선스/안정성 주의

---

## 17. Implementation phases

기준 시각: 2026-06-30 17:19 KST  
참가접수 마감: 2026-07-17 18:00 KST  
출품작 제출 마감: 2026-08-27 18:00 KST  

### Phase 0 — Technical spike, 3 days

**Goal:** 제일 위험한 Windows 기술 검증.

Tasks:

- Rust/Tauri tray app skeleton 생성.
- 투명 `.ico` asset 생성.
- Desktop에 Anchor Shortcut 생성/삭제 proof.
- Shortcut target/action 연결 proof.
- Desktop icon 위치 지정 가능성 검증.
- Overlay window를 바탕화면 위에 띄우기.
- 앱 종료 시 shortcut 삭제 proof.

Exit criteria:

- 2×2 Anchor Shortcut group 생성/삭제 가능.
- overlay card가 해당 영역 위에 보임.
- 클릭 action이 URL 또는 앱을 열 수 있음.
- 기술 리스크 메모 작성.

### Phase 1 — Core app skeleton, 4 days

**Goal:** 실행 가능한 openWidGet shell.

Tasks:

- Rust/Tauri 프로젝트 구조 정리.
- Tray icon/menu.
- Settings window.
- Auto-start toggle.
- config directory/layout file.
- basic logging.
- clean shutdown lifecycle.

Exit criteria:

- Windows 시작 시 실행 가능.
- 트레이에서 종료 가능.
- 종료 시 runtime artifacts 삭제 가능.

### Phase 2 — Live Shortcut Group, 1 week

**Goal:** 위젯이 실제 아이콘 칸을 점유.

Tasks:

- DesktopGridEngine.
- GhostShortcutManager.
- 2×2/4×2/4×4 group support.
- group metadata/sessionId.
- stale shortcut detection.
- layout save/restore.
- batch position commit.

Exit criteria:

- 위젯 instance 생성 → Anchor Shortcut group 생성.
- 위치 변경 → drop 후 group commit.
- 종료 → shortcut 삭제.
- 재실행 → config 기반 재생성.

### Phase 3 — Overlay runtime, 1 week

**Goal:** HTML/CSS/JS 위젯 렌더링.

Tasks:

- WebView overlay.
- WidgetRenderer.
- edit mode/lock mode.
- drag preview.
- grid snap.
- click action dispatch.
- basic widget bridge.

Exit criteria:

- Calendar/Weather sample widget 표시.
- 드래그 후 grid에 snap.
- click으로 URL/app 실행.

### Phase 4 — Widget format and registry, 1 week

**Goal:** 오픈소스 위젯 생태계 구조.

Tasks:

- `widget.json` schema.
- manifest validator.
- local registry.
- install/uninstall.
- permission display.
- widget template.
- registry README.
- GitHub Actions validation draft.

Exit criteria:

- 새 위젯 폴더를 추가하면 gallery에 표시.
- invalid manifest를 거부.
- README/preview/license checks 초안 존재.

### Phase 5 — MVP widget pack, 2 weeks

**Goal:** 데모 가능한 위젯 5개.

Tasks:

- Calendar/Deadline.
- Weather.
- GitHub Repo Status.
- Project Launcher.
- System Monitor.
- Optional Market Watch.

Exit criteria:

- 각 위젯에 preview/README/manifest 존재.
- 권한 표시 가능.
- 실시간 데이터 갱신 가능.
- action dispatch 가능.

### Phase 6 — App detection and gallery UX, 1 week

**Goal:** 필요한 위젯만 가져오는 openWidGet 철학 구현.

Tasks:

- VS Code detection.
- Git detection.
- Browser detection.
- Docker detection optional.
- recommendation cards.
- gallery install/uninstall UI.
- installed/enabled/running status.

Exit criteria:

- “VS Code가 설치되어 있어요. Project Widget을 설치할까요?” flow 작동.
- 선택 설치/삭제 가능.

### Phase 7 — Hardening, 1 week

**Goal:** 제출 가능한 안정성.

Tasks:

- Startup restore QA.
- Shutdown cleanup QA.
- Crash/stale shortcut recovery QA.
- OneDrive Desktop warning.
- display/DPI change basic recovery.
- network failure/stale data UI.
- resource usage check.
- secret-like scan.
- SBOM generation.

Exit criteria:

- Known issues 문서화.
- 주요 smoke checklist 통과.
- 데모 영상 촬영 가능한 상태.

### Phase 8 — Submission package, 3 days

**Goal:** 대회 제출물 완성.

Tasks:

- README.
- Architecture doc.
- Widget developer guide.
- Result report 5 pages.
- PDF + original doc.
- 3-minute demo video.
- Public GitHub repo.
- LICENSE.
- SBOM.
- screenshots.

Exit criteria:

- 제출물 누락 없음.
- 데모 영상에서 핵심 차별점이 30초 안에 보임.

---

## 18. Recommended repository structure

```txt
openWidGet/
├─ README.md
├─ LICENSE
├─ docs/
│  ├─ architecture.md
│  ├─ product-plan.md
│  ├─ widget-manifest.md
│  ├─ security-model.md
│  └─ contest-submission.md
├─ apps/
│  └─ openwidget-desktop/
│     ├─ src-tauri/
│     └─ ui/
├─ crates/
│  ├─ openwidget-core/
│  ├─ openwidget-shell/
│  ├─ openwidget-shortcuts/
│  ├─ openwidget-registry/
│  └─ openwidget-auth/
├─ widgets/
│  ├─ clock/
│  ├─ calendar-deadline/
│  ├─ weather/
│  ├─ github-status/
│  ├─ project-launcher/
│  └─ system-monitor/
├─ registry/
│  ├─ registry.json
│  └─ widgets/
├─ examples/
├─ scripts/
└─ tests/
```

Note: 브랜드 표기는 `openWidGet`, package/crate/CLI는 lowercase `openwidget` 권장.

---

## 19. Verification plan

### 19.1 Core smoke tests

- App starts from tray.
- Auto-start setting can be enabled/disabled.
- App creates Anchor Shortcut group.
- App deletes Anchor Shortcut group on exit.
- App restores layout on restart.
- Stale shortcuts are removed/repaired on startup.

### 19.2 UI smoke tests

- Overlay appears above desktop region.
- Edit mode shows grid.
- Drag preview is smooth.
- Drop commits shortcut positions.
- Lock mode prevents accidental movement.
- Widget click opens configured action.

### 19.3 Widget runtime tests

- Valid manifest accepted.
- Invalid manifest rejected.
- Permission display matches manifest.
- Network permission blocks disallowed domains.
- Hidden widget stops refresh.
- Offline state displays cached/stale data.

### 19.4 Performance checks

- Idle CPU approximately 0~1%.
- Normal RAM budget under target, e.g. 200MB preferred.
- Drag does not move real shortcuts per frame.
- Drop commit with 20~40 anchors is acceptable.
- API refresh intervals do not spam network.

### 19.5 Security checks

- No token in widget JS/localStorage.
- No command execution without explicit permission.
- Community widget validation rejects banned APIs.
- Logs redact secrets.
- OAuth tokens stored by Rust Core only.

---

## 20. Contest report angle

보고서 핵심 문장:

> openWidGet은 Windows 바탕화면의 정적인 아이콘 그리드를 Live Shortcut Group으로 확장하여, 위젯이 실제 아이콘 공간을 점유하면서도 HTML/CSS/JS 기반 실시간 UI를 제공하는 오픈소스 데스크탑 위젯 플랫폼이다. 사용자는 설치된 앱과 작업환경에 맞춰 필요한 위젯만 가져오고, 개발자는 manifest 기반 위젯을 PR로 기여할 수 있다.

강조할 평가 포인트:

- 오픈소스 생태계성: PR-first widget registry.
- 기술 차별성: icon-backed widget / Anchor Shortcut substrate.
- 실용성: 개발자/학생 워크플로우 위젯.
- 보안성: 권한 manifest, OAuth token isolation.
- 확장성: on-demand widget install, app detection recommendations.
- 데모성: 바탕화면에서 바로 보이는 시각 임팩트.

---

## 21. Immediate next steps

1. **Repo/workspace 생성**
   - Path proposal: `/Volumes/SSD/JinuVault/Hackathons/openWidGet/openWidGet`
   - GitHub repo name proposal: `openWidGet` or `openwidget`

2. **Phase 0 spike 시작**
   - Windows 환경 필요.
   - Rust/Tauri skeleton.
   - Anchor Shortcut 생성/삭제 proof.
   - Overlay proof.

3. **Designer artifact 생성**
   - iOS/macOS-style widget visual mock.
   - Windows desktop screenshot composite.
   - Edit mode + Gallery mock.

4. **PM spec freeze**
   - 위 문서에서 MVP/Non-goals 확정.
   - 확정 후 Kanban card decomposition.

5. **Implementation route**
   - Project-grade implementation이므로 Kanban decomposition 후 coder route 권장.
   - Reviewer gate는 Phase 0 spike와 Phase 3 runtime 후 각각 필요.

---

## 22. Key decisions already made

- Name: **openWidGet**.
- Meaning: Open Widget + Get.
- Platform: Windows desktop.
- Main engine: Rust.
- App shell: Tauri/WebView2 preferred.
- Widget UI: HTML/CSS/JS.
- Widget packaging: `widget.json` manifest + web files.
- Widget install: on-demand, not all bundled.
- Licensing: core/runtime AGPL-3.0-or-later; widget templates/examples MIT; community widgets OSI-approved license required.
- App detection: recommend widgets based on installed apps.
- Startup: auto-start recommended, user-controllable.
- Background: tray-based lightweight agent.
- Ghost shortcut policy: runtime-only; created on app start/show, deleted on exit/hide/delete.
- Drag performance policy: overlay moves during drag, shortcuts move only on drop commit.
- OAuth: Rust Core handles auth/token storage; widget JS never sees tokens.

---

## 23. Open questions

- Exact Windows API path for reliable desktop icon positioning in Rust.
- Whether Tauri window control is sufficient or custom WebView2/Win32 shell is needed.
- Minimum supported Windows version: Windows 11 only or Windows 10+.
- Multi-monitor support in v0.1 or v0.2.
- Whether Anchor Shortcut count should be every grid cell or optimized edge/corner anchors.
- Public API choices for Weather/Market widgets.
- Exact GitHub OAuth strategy: public-only first, token input, device flow, or PKCE.
- Branding casing in code/package: `openWidGet` display, `openwidget` technical.

---

## 24. MVP acceptance criteria

MVP is acceptable when a demo can show:

- Windows startup/tray openWidGet running.
- Add a 2×2 Calendar widget.
- Add a 2×2 Weather widget.
- Add a 4×2 GitHub widget.
- Widgets occupy desktop icon grid via Anchor Shortcut groups.
- Edit mode drag/snap works without visible lag.
- Dropping a widget commits Anchor Shortcut positions.
- Lock mode makes widgets feel like desktop-native elements.
- Clicking a widget opens configured app/site.
- App exit removes Anchor Shortcuts.
- App restart restores widgets from config.
- Gallery installs at least one non-bundled widget.
- App detection recommends at least one widget, e.g. VS Code Project Launcher.
- README explains how to contribute a widget by PR.

If these are shown, the project is meaningfully different from generic desktop widget apps.
