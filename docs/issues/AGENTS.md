## Journaling

If working on a feature branch, always keep notes on decisions, thoughts and code validated by the user. Think of the journal as shared memory with the developer for the current task. Any breakthroughs or changes of scope should be noted here.

This allows us to keep track of design decisions and rationale in one place, and makes it easier to review and iterate on the design as we implement the feature.

Follow these rules for journaling:

1. Always keep a journal when working on a branch different than `main` and use this as a base template:

    ```markdown
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

    <!-- (Describe the problem, constraints, and goals here) -->

    ## Tasks

    - [ ] ...<!-- (List the high-level tasks you will take to implement the feature here) -->
        - [ ] ...<!-- (Break down the tasks into smaller steps as you make progress) -->
    

    # Journal

    <!-- >Update this section as you make progress on the implementation -->

    ## {task} ({YYYY-MM-DD HH:mm})

    1. **{YYYY-MM-DD HH:mm}**: ... <!-- Describe the thought, decision, or code change here, make sure you reflect the users intentions, rationale, feedback, blockers and pivots -->
        2. ... <!-- Add more points as needed sub steps -->

    ```

2. Always inquire if the user wants to add something to the journal after the following events during conversation:

    1. A succesful logical code change
    2. A complete hypothesis is formed, tested, disproven or validated.

3. When starting a new session, always check the journal first as a starting point for your thinking.

4. The journal is always named after the branch, since this is a Github project (i.e. `{github_id}-{issue_name}`), the journal always goes in this structure `docs/issues/{issue_type}/{branch_name|}/#{github_id}.md` (create the file if it doesn't exist). 

5. It is OK to edit the journal after the fact, even when in planning mode, but prompt the user.

6. Whenever deciding not to do a task, don't remove it, cross it out instead.