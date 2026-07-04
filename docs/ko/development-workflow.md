# 개발 워크플로우

OpenWidGet은 이제 **이슈별 PR** 방식으로 관리합니다.

## 원칙

- 의미 있는 변경은 먼저 GitHub Issue로 정의합니다.
- 하나의 브랜치/PR은 하나의 primary issue를 기준으로 합니다.
- PR은 작게 유지합니다. 범위가 커지면 follow-up issue를 새로 만듭니다.
- 검증 로그, 스크린샷, Windows 수동 테스트 결과도 deliverable의 일부입니다.
- `main` 직접 커밋은 긴급/관리성 변경에만 예외적으로 사용합니다.

## 흐름

```text
Issue 작성
→ issue 번호 기반 branch 생성
→ 구현/문서/디자인 반영
→ PR 생성, Closes #issue 연결
→ 검증/evidence 첨부
→ review/PM gate
→ merge
```

## 브랜치 예시

```text
issue-28-alpha-shell-visual-direction
fix-20-windows-smoke-blocker
docs-29-issue-pr-workflow
```

## PR에 포함할 것

- 연결된 issue 번호
- 변경 요약
- scope / out-of-scope
- 실행한 검증 명령
- UI/Windows 동작 변경이면 스크린샷 또는 수동 테스트 결과
- 남은 risk와 follow-up issue
