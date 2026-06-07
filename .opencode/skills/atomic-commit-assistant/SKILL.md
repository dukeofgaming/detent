---
name: atomic-commit-assistant
description: git commit, staged changes, unstaged changes, atomic commit selection, test-before-commit, AI author plus-address. Use when the developer wants OpenCode to turn working tree changes into one or more safe commits.
---

# Atomic Commit Assistant

Use ONLY when the developer wants help reviewing changes and producing one or more git commits.

## Outcomes

- Produce `1+` commits.
- Keep each commit atomic.
- Always confirm each commit with the developer before running `git commit`.
- Never modify project code, configuration, prompts, or documentation while invoking this command.

## Required Workflow

1. Inspect the current git state before planning a commit.
   - Review staged and unstaged changes separately.
   - Prefer `git status --short`, `git diff --staged`, `git diff`, and `git log --oneline -10`.
   - Treat this command as commit-only: inspect, stage, test, and commit existing changes, but do not rewrite files.
2. Decide whether the staged area already contains one atomic change.
   - If yes, write a concise commit summary for that staged change.
   - If no, suggest a split into multiple commits and ask the developer which commit to do first.
3. If there are no staged changes, stage only the files or hunks needed for the next atomic commit.
   - Do not pull unrelated changes into the commit.
   - If a safe non-interactive partial stage is not practical, explain the split and ask the developer how to proceed.
   - Do not edit files to make them easier to commit.
4. Determine whether the upcoming staged diff is AI-only and untouched by the developer.
   - Only treat the change as AI-only when there is strong evidence that the agent wrote the staged scope and the developer did not modify it afterward.
   - If there is any uncertainty, treat it as developer-authored and use the normal author identity.
5. If tests are present for that atomic change, run them before committing.
   - Choose the smallest relevant test command that gives meaningful coverage.
   - If no relevant tests exist, say so explicitly before asking for commit confirmation.
   - If tests fail, do not commit until the developer decides how to proceed.
6. Before each commit, present a confirmation message to the developer that includes:
   - The proposed commit subject and short summary.
   - The files or scope included.
   - Any important unstaged changes being left out.
   - The test command and result, or the reason no test was run.
   - The author name and email that will be used.
7. Only after explicit developer confirmation, create the commit.
8. After each commit, reassess the remaining staged and unstaged changes and continue until there are no more atomic commits to make or the developer stops.

## Commit Splitting Rules

- Prefer the smallest correct commit.
- Separate refactors from behavior changes when practical.
- Separate generated or mechanical changes from hand-written logic changes when they are independently understandable.
- If staged changes mix unrelated concerns and the first atomic commit is not obvious, stop and ask which suggested commit should be made first.

## Commit Summary Requirements

For each proposed commit, provide:

- A one-line subject the developer could use as the commit title.
- A 2-4 bullet summary of what changed and why.
- A note about what is intentionally excluded from this commit.

## AI Author Email Rule

When, and only when, the staged scope for the next atomic commit was written by AI and left untouched by the developer, keep the normal author name but rewrite the author email using a plus address with the normalized active model name.

Examples:

- `something@domain.com` -> `something+gpt-5.4@domain.com`
- `something+tag@domain.com` -> `something+tag+gpt-5.4@domain.com`

Use the rewritten address only for that single commit. All other commits should use the normal configured author identity.

## Safety Rules

- Never commit without explicit developer confirmation.
- Never modify files as part of `/commit`; if something looks wrong, stop and ask instead of editing it.
- Never assume mixed staged changes are acceptable as one commit.
- Never amend, force-push, or rewrite history unless the developer explicitly asks.
- Never claim AI-only authorship when the evidence is incomplete.
- If existing staged changes appear intentional but do not match the proposed atomic commit, ask before changing the staging area.

## Suggested Confirmation Template

Use a confirmation message like this before each commit:

```text
Proposed commit: <subject>

Summary:
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
