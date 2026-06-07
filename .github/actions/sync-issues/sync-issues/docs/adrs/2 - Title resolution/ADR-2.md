---
type: adr
title: ADR-2 - Title Resolution Order
date: 2026-04-07
status: accepted
supersedes: 
---

## Context

Issue titles can come from multiple sources: frontmatter `title` field, filename (e.g., `#3.md` → "3"), or parent folder name (e.g., `3-convert-bpmn-to-mdx/#3.md` → "Convert BPMN to MDX"). The canonical template in `docs/issues/AGENTS.md` specifies title in frontmatter, but we need sensible fallbacks. This ADR is related to ADR-001 for issue matching.

## Decision

Title resolution follows a four-tier order:

1. **Primary**: `issue.title` from frontmatter.
2. **Secondary**: Extract from first H1 heading (`# Title`) in the markdown content.
3. **Tertiary**: Parse from folder name:
   - `3-feature-name/` → "Feature: feature-name" (capitalize, replace hyphens).
   - `bugfix/1-foo/` → "Fix: foo".
4. **Final**: Use filename without extension (least preferred).

### Options

- **Frontmatter-only**: Require every issue file to have an explicit `issue.title` in frontmatter. Most reliable but creates friction for quick issue creation.
- **Filename-only**: Use the filename as the issue title. Simple but loses semantic meaning (e.g., `#3.md` yields "3").
- **Folder-name parsing (chosen as tertiary)**: Derive title from the parent folder slug. Preserves semantic intent encoded in conventional directory names.
- **Tiered resolution (chosen)**: Try each source in priority order, stopping at the first available value.

### Rationale

A tiered resolution balances reliability and convenience. Frontmatter is the most explicit and authoritative source, so it takes top priority. The H1 heading is a natural convention in markdown and usually carries the intended title. Folder name parsing recovers semantic meaning from the directory structure when frontmatter and headings are absent. The filename is a last-resort fallback that guarantees a title always exists. This ordering ensures the richest available source is used without blocking sync for files missing optional metadata.

## Consequences

### Positive

1. Works with minimal frontmatter — files don't need an explicit title field.
2. Parent folder provides semantic context via slug parsing (e.g., "Feature: feature-name").
3. Each tier degrades gracefully to the next, ensuring every issue always gets a title.

### Negative

1. Multiple folder naming conventions must be handled (e.g., `3-feature-name/` vs. `bugfix/1-foo/`).
2. Folder-name parsing can produce misleading titles if the slug doesn't follow conventions.
3. Increasing complexity in title resolution logic compared to a single-source approach.
