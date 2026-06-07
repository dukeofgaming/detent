# Sync Issues Plan

## Overview

Refactor the sync-issues script to properly:
1. Match issue files to GitHub issues by filename (`#<id>.md` maps to GH issue `#<id>`)
2. Use the canonical journal template from `docs/issues/AGENTS.md`
3. Extract title from frontmatter `issue.id`, fallback to filename, then parent folder
4. Map sections to comments correctly

## Architecture

Vertical slice: `push-github`
Clean Architecture layers: domain, application, adapter, infrastructure

```
.github/actions/sync-issues/sync-issues/
├── domain/           # Core business logic & types
│   └── types/        # Issue, Journal, Section, etc.
├── application/      # Use cases
│   └── usecases/     # SyncIssue, ParseIssueFile
├── adapter/          # Interfaces & ports
│   ├── github/       # GitHub issue adapter interface
│   └── filesystem/   # File system adapter interface
├── infrastructure/   # External dependencies
│   └── gh-cli/       # Concrete gh CLI adapter
├── docs/
│   ├── plan.md       # This file
│   └── adrs/         # Architectural Decision Records
└── index.mjs         # Entry point
```

## Key Decisions (ADRs)

- [[001-issue-file-matching]] - Issue file matching strategy (filename → GH issue ID)
- [[002-title-resolution]] - Title resolution order (frontmatter → filename → folder)
- [[003-section-comment-mapping]] - Section-to-comment mapping
- [[004-idempotency-strategy]] - Idempotency strategy
- [[005-folder-structure]] - Folder structure (vertical slice + clean architecture)

## Implementation Tasks

### Phase 1: Domain Layer
- [ ] Define `IssueFile` type with all frontmatter fields
- [ ] Define `Section` type (description, tasks, journal)
- [ ] Define `GitHubIssue` type
- [ ] Implement `parseIssueFile(content)` function
- [ ] Implement `extractSections(content)` function

### Phase 2: Application Layer
- [ ] Create `SyncIssueUseCase`
- [ ] Implement issue matching logic
- [ ] Implement section diffing for idempotency

### Phase 3: Adapter Layer
- [ ] Define `IssueAdapter` interface
- [ ] Define `FileAdapter` interface

### Phase 4: Infrastructure Layer
- [ ] Implement `GhCliAdapter` using gh CLI
- [ ] Implement `NodeFileAdapter` using fs module

### Phase 5: Integration
- [ ] Wire up index.mjs entry point
- [ ] Update workflow to use new structure

## Current Issues to Fix

| File | Current Behavior | Expected Behavior |
|------|-----------------|-------------------|
| `3-convert-bpmn-to-mdx/#3.md` | Created new issue #4 | Update existing issue #3 |
| `4-graph-validation/#4.md` | Created new issue #4 | (new issue, correct) |

### Root Cause

The file `#3.md` does not have `issue.id: 3` in frontmatter, so the script:
1. Extracted title "Feature #3: Convert BPMN to MDX" from H1
2. Couldn't find an issue with that exact title
3. Created a new issue #4

### Fix Required

Update frontmatter in each issue file to include:
```yaml
---
issue:
    id: 3  # Maps to GH issue #3
---
```

Then re-run the sync script to update #3 correctly.

## References

- [Template](../../../../../docs/issues/AGENTS.md)
- [gh issue create docs](https://cli.github.com/manual/gh_issue_create)
- [gh issue comment docs](https://cli.github.com/manual/gh_issue_comment)
