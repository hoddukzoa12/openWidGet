# Windows 수동 테스트 체크리스트

## 첫 실행 테스트

Rust/Cargo가 먼저 잡혀야 합니다.

```powershell
rustc --version
cargo --version
```

인식되지 않으면:

```powershell
winget install --id Rustlang.Rustup -e
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

설치 후 PowerShell을 새로 열고 다시 테스트합니다.

```powershell
git clone https://github.com/hoddukzoa12/openWidGet.git
cd openWidGet
npm install
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri:dev
```

## 확인할 것

- 앱 창이 뜨는가?
- Clock/Memo/Project Launcher 카드가 보이는가?
- Refresh status 버튼을 눌러도 앱이 유지되는가?
- tray 아이콘/메뉴가 보이는가?
- 종료 후 이상한 바로가기나 임시 파일이 남지 않는가?

## 보내줄 결과 형식

```text
목표: OpenWidGet skeleton launch
결과: 성공/실패
증상: 보인 화면 또는 오류
로그: PowerShell 출력
첨부: 스크린샷/영상
```
