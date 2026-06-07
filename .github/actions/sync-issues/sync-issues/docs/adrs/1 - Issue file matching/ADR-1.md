---
type: adr
title: ADR-1 - Issue File Matching Strategy
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

Issue files in `docs/issues/` are named `#<id>.md` where `<id>` should correspond to a GitHub issue number. The current implementation matches issues by title, which causes new issues to be created instead of updating existing ones, and the filename is not being used to determine the target GH issue. The canonical template is defined in `docs/issues/AGENTS.md`.

## Decision

We will use a three-tier matching strategy:

1. **Primary**: If `issue.id` is present in frontmatter, use that GH issue number directly.
2. **Secondary**: Extract ID from filename pattern `#<number>.md`.
3. **Fallback**: Match by title (current behavior, for files without ID).

This ensures that `#3.md` maps to GH issue #3, `#4.md` maps to GH issue #4 (or creates #4 if it doesn't exist), and files without numeric prefix fall back to title matching.

### Options

- **Title-only matching**: Match issues by title alone. Simple but causes duplicate issues when filenames change and fails to leverage the already-known issue number in the filename.
- **Filename-only matching**: Use only the `#<N>.md` pattern to determine the target issue. Would break for files that don't follow the naming convention.
- **Multi-tier matching (chosen)**: Use frontmatter ID first, then filename pattern, then title fallback. Provides deterministic behavior while gracefully degrading for files that lack metadata.

### Rationale

A three-tier approach provides the most robust matching. Frontmatter `issue.id` is the most authoritative source (explicit metadata). Extracting the ID from the filename is a practical default since the naming convention already encodes the issue number. Title matching serves as a safety net for files created before this convention was adopted. This ordering ensures that as much of the file metadata is leveraged as possible before falling back to the least reliable method.

## Consequences

### Positive

1. Existing issues are updated correctly rather than duplicated.
2. Clear, predictable mapping between files and GH issues.
3. Graceful degradation — files without numeric prefixes or frontmatter IDs still work.

### Negative

1. Filenames must be kept in sync with GH issue numbers to avoid mismatches.
2. Title-based fallback can still produce false matches if titles collide.
