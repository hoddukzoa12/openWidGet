# OpenWidGet

> HTML/CSS/JS로 만드는 Windows 데스크톱 위젯 런타임

[English README](./README.md)

OpenWidGet은 Windows 바탕화면 위젯을 더 쉽게 만들고 공유할 수 있게 하는 오픈소스 실험 프로젝트입니다.

## 목표

- Windows에서 실행 가능한 데스크톱 위젯 런타임 만들기
- HTML/CSS/JS 기반으로 위젯을 작성할 수 있게 하기
- 초기에는 작은 샘플 위젯과 실행 가능한 `.exe` 릴리즈를 제공하기
- 이후 커뮤니티가 PR로 위젯을 추가할 수 있는 구조로 발전시키기

## 현재 상태

`v0.1-alpha`를 준비 중입니다. 현재 단계는 완성형 플랫폼이 아니라 앱 스켈레톤과 위젯 프리뷰를 검증하는 초기 버전입니다.

## 실행

```bash
npm install
npm run build
npm run tauri:dev
```

Windows 직접 테스트 체크리스트는 [`docs/ko/manual-test.md`](./docs/ko/manual-test.md)를 참고하세요.

## 문서

- [한국어 Quickstart](./docs/ko/quickstart.md)
- [프로젝트 노트](./docs/ko/project-note.md)
- [수동 테스트](./docs/ko/manual-test.md)
- [English README](./README.md)

## 라이선스

- Core/runtime/desktop app: AGPL-3.0-or-later
- Widget templates/example starter code: MIT
- Community widgets: `widget.json`에 선언된 OSI-approved license
