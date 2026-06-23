---
name: atomic-commit-assistant
description: git commit, staged changes, unstaged changes, atomic commit selection, test-before-commit, AI author plus-address. The agent must never auto-commit. After every logical code change, run all tests and if they pass, propose a commit with a short 1-line summary and a detailed summary, then ask the developer for confirmation.
---

# Atomic Commit Assistant

Use when the developer has finished a logical code change, or when the `/commit` command is invoked. The agent must never commit automatically.

## Outcomes

- Propose atomic commits after work is complete and tests pass.
- Never commit without explicit developer confirmation.
- Keep each commit atomic.
- Never modify project code, configuration, prompts, or documentation while invoking this command.

## Required Workflow

1. After completing a logical code change, run the full test suite (`cargo test`).
   - If tests fail, do not proceed to commit; report failures and wait for the developer.
   - If no tests exist for the change set, say so explicitly.
2. Only when all tests pass, inspect the current git state.
   - Review staged and unstaged changes separately.
   - Prefer `git status --short`, `git diff --staged`, `git diff`, and `git log --oneline -10`.
   - Treat this as commit-only: inspect, stage, test, and commit existing changes, but do not rewrite files.
3. Present a commit proposal to the developer:
   - A one-line summary (suitable as a commit title).
   - A detailed summary of what changed and why.
   - The files or scope included.
   - Any important unstaged changes being left out (if any).
   - The test command and result.
   - If the change is AI-only, the author email with plus-address suffix.
4. If staged changes mix unrelated concerns, suggest splitting into multiple commits and ask which to do first.
5. If there are no staged changes but unstaged changes exist, stage only the files or hunks needed for the next atomic commit.
   - Do not pull unrelated changes into the commit.
   - If a safe non-interactive partial stage is not practical, explain the split and ask the developer how to proceed.
   - Do not edit files to make them easier to commit.
6. Determine whether the upcoming staged diff is AI-only and untouched by the developer.
   - Only treat the change as AI-only when there is strong evidence that the agent wrote the staged scope and the developer did not modify it afterward.
   - If there is any uncertainty, treat it as developer-authored and use the normal author identity.
7. Only after explicit developer confirmation, create the commit.
8. After each commit, reassess the remaining staged and unstaged changes and continue until there are no more atomic commits to make or the developer stops.

## Commit Splitting Rules

- Prefer the smallest correct commit.
- Separate refactors from behavior changes when practical.
- Separate generated or mechanical changes from hand-written logic changes when they are independently understandable.
- If staged changes mix unrelated concerns and the first atomic commit is not obvious, stop and ask which suggested commit should be made first.

## Commit Summary Requirements

For each proposed commit, provide:

- **Summary** (one line): A short, direct line suitable as the commit title.
- **Details**: A bulleted breakdown of what changed and why (2-4 bullets).
- A note about what is intentionally excluded from this commit.

## AI Author Email Rule

When, and only when, the staged scope for the next atomic commit was written by AI and left untouched by the developer, keep the normal author name but rewrite the author email using a plus address with the normalized active model name.

Examples:

- `something@domain.com` -> `something+gpt-5.4@domain.com`
- `something+tag@domain.com` -> `something+tag+gpt-5.4@domain.com`

Use the rewritten address only for that single commit. All other commits should use the normal configured author identity.

## Safety Rules

- Never commit without explicit developer confirmation. Never auto-commit.
- Never modify files as part of `/commit`; if something looks wrong, stop and ask instead of editing it.
- Never assume mixed staged changes are acceptable as one commit.
- Never amend, force-push, or rewrite history unless the developer explicitly asks.
- Never claim AI-only authorship when the evidence is incomplete.
- If existing staged changes appear intentional but do not match the proposed atomic commit, ask before changing the staging area.

## Suggested Confirmation Template

Use a confirmation message like this before each commit:

```text
Proposed commit: <one-line summary>

Details:
- ...
- ...

Included scope:
- <file or area>

Left out for later:
- <file or area>

Tests:
- <command>: <passed | failed | not run: reason>

Author:
- <name> <email>

Commit this now?
```
