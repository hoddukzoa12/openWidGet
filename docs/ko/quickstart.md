# OpenWidGet Quickstart

## 준비물

- Node.js 22+
- npm 11+
- Rust stable
- Windows 테스트 시 Edge WebView2 Runtime

## Rust/Tauri 준비

`npm run build`는 프론트엔드만 빌드하므로 Rust 없이도 통과할 수 있습니다. `npm run tauri:dev`는 Rust/Cargo가 필요합니다.

PowerShell에서 `rustc` 또는 `cargo`가 인식되지 않으면 먼저 설치합니다.

```powershell
winget install --id Rustlang.Rustup -e
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

설치 후 **PowerShell을 완전히 새로 열고** 확인합니다.

```powershell
rustc --version
cargo --version
```

그래도 안 잡히면 현재 세션에 임시 PATH를 추가합니다.

```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
rustc --version
cargo --version
```

## 실행

```powershell
git clone https://github.com/hoddukzoa12/openWidGet.git
cd openWidGet
npm install
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri:dev
```

## 기대 결과

- OpenWidGet 창이 뜸
- Clock, Memo, Project Launcher 프리뷰 카드가 보임
- Refresh status 버튼이 동작함
- 가능하면 tray 메뉴가 보임

문제가 생기면 PowerShell 출력과 스크린샷을 저장합니다.
