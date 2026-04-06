# TODO: GitHub Issue Sync Action

## Overview

Create a GitHub composite action that syncs issues from `docs/issues/` markdown files to GitHub Issues using the `gh` CLI.

## Directory Structure

```
.github/
└── actions/
    └── sync-issues/
        ├── action.yml          # Composite action definition
        ├── push.sh             # Shell entrypoint (wraps node)
        └── sync-issues.mjs     # Main Node.js implementation (ESM)
```

## Architecture

### Input Schema (from issue files)

Issue files use frontmatter:

```yaml
---
type: issue
title: {issue_title}
author: {github_username}
created: {YYYY-MM-DD HH:mm}
updated: {YYYY-MM-DD HH:mm}
issue:
    tags: 
        - {some_label}
        - {some_tag}: {some_value}
    sections:
        - description: {github_comment_id}
        - tasks: {github_comment_id}
        - journal: {github_comment_id}
---

## Description

...

## Tasks

- [ ] ...

# Journal

...
```

### Output Mapping (to GitHub)

| Source Section | GitHub Target |
|----------------|---------------|
| Title + Description (before Tasks) | Issue body (first comment) |
| Tasks section | GitHub comment |
| Journal section | GitHub comment |

### Idempotency Strategy

1. **Issue matching**: Use exact title match to detect existing issues
2. **Comment tracking**: Store `github_comment_id` in frontmatter `sections` mapping
3. **Update vs Create**:
   - Existing issue with tracked comments → Update each section's comment
   - Existing issue without tracking → Create new comments (append)
   - New issue → Create with all sections as comments

### CLI Interface

```bash
# Push issues to GitHub
node sync-issues.mjs push [--dry-run]

# Show help
node sync-issues.mjs help
```

## Tasks

### Phase 1: Core Implementation

- [ ] Create `sync-issues.mjs` with:
  - [ ] `parseIssueFile(filepath)` - Parse frontmatter + markdown sections
  - [ ] `parseFrontmatter(content)` - Extract YAML frontmatter
  - [ ] `findExistingIssue(title)` - Query GitHub API for matching issue
  - [ ] `createIssue(issue)` - Create issue + comments via `gh`
  - [ ] `updateIssueComments(issue, existingComments)` - Update tracked comments
  - [ ] `buildIssueBody(parsed)` - Extract description from markdown
  - [ ] `buildSectionComments(parsed)` - Extract Tasks, Journal, etc.

- [ ] TDD approach: Inline tests using Node.js built-in test runner
  - [ ] Test frontmatter parsing
  - [ ] Test section extraction
  - [ ] Test body building
  - [ ] Test idempotency logic

### Phase 2: CLI Interface

- [ ] `main()` function with command dispatch
- [ ] `push` command: Process all issues and sync to GitHub
- [ ] `help` command: Display usage information
- [ ] `--dry-run` flag: Show what would be created without making changes

### Phase 3: GitHub Actions Integration

- [ ] Create `action.yml` with:
  - [ ] `inputs.token` - GitHub token (for gh CLI)
  - [ ] `inputs.dry-run` - Boolean flag
  - [ ] `outputs` - Summary of changes
- [ ] Create `push.sh` entrypoint
- [ ] Create workflow file `.github/workflows/sync-issues.yml`

### Phase 4: Edge Cases & Polish

- [ ] Handle missing `type:` in frontmatter
- [ ] Handle malformed YAML frontmatter
- [ ] Handle issues without Tasks or Journal sections
- [ ] Verbose logging with `--verbose`
- [ ] Error handling with meaningful exit codes

## gh CLI Commands (Verified)

```bash
# Create issue
gh issue create --title "Title" --body "Body" --label "label1" --label "label2"

# List issues (for matching)
gh issue list --state all --limit 100 --json number,title

# Create comment
gh issue comment {number} --body "Comment body"

# Edit comment (requires API)
gh api repos/{owner}/{repo}/issues/comments/{id} -X PATCH --field body="Updated body"
```

## Non-Goals

- Bidirectional sync (GitHub → local) - only push local → GitHub
- Label management (reuse existing labels)
- Issue deletion
- Moving comments between issues

## References

- [gh issue create docs](https://cli.github.com/manual/gh_issue_create)
- [gh issue comment docs](https://cli.github.com/manual/gh_issue_comment)
- [gh api docs](https://cli.github.com/manual/gh_api)
