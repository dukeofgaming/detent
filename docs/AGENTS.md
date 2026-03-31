## Journaling

If working on a feature branch, always keep notes on decisions, thoughts and code validated by the user. Think of the journal as shared memory with the developer for the current task. Any breakthroughs or changes of scope should be noted here.

This allows us to keep track of design decisions and rationale in one place, and makes it easier to review and iterate on the design as we implement the feature.

Follow these rules for journaling:

1. Always keep a journal when working on a branch different than `main`
2. Always inquire if the user wants to add something to the journal after the following events during conversation:

    1. A succesful logical code change
    2. A complete hypothesis is formed, tested, disproven or validated.

3. When starting a new session, always check the journal first as a starting point for your thinking.

4. The journal is always named after the branch, since this is a Github project (i.e. `{github_id}-{issue_name}`), the journal always goes in this structure `docs/notes/features/{branch_name}/#{github_id}.md` (create the file if it doesn't exist). 

5. It is OK to edit the journal after the fact, even when in planning mode, but prompt the user.