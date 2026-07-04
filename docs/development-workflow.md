# Development Workflow

OpenWidGet uses an issue-driven pull request workflow.

## Principles

- One primary issue per branch and PR.
- Small PRs are preferred over broad mixed changes.
- The issue defines the product/technical scope; the PR proves the implementation.
- Verification evidence is part of the deliverable.
- Direct `main` commits are reserved for emergency/admin-only changes.

## Flow

### 1. Create or select an issue

Every non-trivial change should start from a GitHub Issue with:

- goal;
- scope and non-scope;
- acceptance criteria;
- required evidence;
- owner route: PM/default, designer, coder, reviewer, Jinu manual Windows, or mixed.

### 2. Create a branch

Use a branch name that keeps the issue visible:

```text
issue-<number>-short-description
fix-<number>-short-description
docs-<number>-short-description
```

Examples:

```text
issue-28-alpha-shell-visual-direction
fix-20-windows-smoke-blocker
docs-29-issue-pr-workflow
```

### 3. Implement within scope

Keep the PR limited to the issue. If you discover extra work, create follow-up issues instead of silently expanding the PR.

For implementation cards, include an Evidence Pack when possible:

```bash
python3 /Users/hoddukzoa/.hermes/scripts/evidence_pack.py \
  --repo /Volumes/SSD/JinuVault/Projects/openWidGet \
  --cmd 'npm run build && cargo check --manifest-path src-tauri/Cargo.toml' \
  --note 'card=#<issue>; tdd=<yes/no>'
```

### 4. Open the PR

The PR should include:

- linked issue: `Closes #<issue>` or `Refs #<issue>`;
- summary of changes;
- verification commands and output summary;
- screenshots or manual Windows evidence when user-facing;
- risks and non-goals.

### 5. Review and merge

Before merge:

- changed files match the issue scope;
- required verification is green or blockers are explicit;
- screenshots/manual evidence exist for desktop UX changes;
- reviewer/PM gate is satisfied;
- follow-up issues exist for deferred work.

## Current v0.1-alpha gates

- Runtime/shell work: must pass `npm run build` and Rust/Tauri checks.
- Windows shell behavior: requires real Windows smoke evidence from Jinu or reviewer.
- Visual/public-demo changes: should be designer-led and include screenshots.
- Release packaging: requires Windows artifacts plus checksum and unsigned-app notice.
