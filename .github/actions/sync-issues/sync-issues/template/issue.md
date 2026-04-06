---
type: issue
title: {issue_title}
author: {github_username}
created: {YYYY-MM-DD HH:mm}
updated: {YYYY-MM-DD HH:mm}
issue:
    -- `id` is optional
    id: {github_issue_id}
    tags: 
        - {some_label}
        - {some_tag}: {some_value}
    sections:
        - description: {github_comment_id}
        - tasks: {github_comment_id}
        - journal: {github_comment_id}
---

## Description

<!-- (Describe the problem, constraints, and goals here), this matches the GitHub issue description when synced -->

## Tasks <!-- URL: {synced_tasks_comment_url} -->

- [ ] ...<!-- (List the high-level tasks you will take to implement the feature here) -->
    - [ ] ...<!-- (Break down the tasks into smaller steps as you make progress) -->


# Journal <!-- URL: {synced_journal_comment_url} -->

<!-- >Update this section as you make progress on the implementation -->

## {task} ({YYYY-MM-DD HH:mm})

1. **{YYYY-MM-DD HH:mm}**: ... <!-- Describe the thought, decision, or code change here, make sure you reflect the users intentions, rationale, feedback, blockers and pivots -->
    2. ... <!-- Add more points as needed sub steps -->
