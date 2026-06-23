## ADR Compliance

All code changes in this slice must follow the ADRs in `docs/adrs/`.

### Before making changes

1. Read the relevant ADRs to understand current decisions
2. If your change contradicts an ADR, note which ADR and how it conflicts
3. Consider if the ADR should be updated instead

### When ADR is broken

If you decide to break an ADR (e.g., add cross-slice imports, combine multiple types in one file):

1. Document the violation in your response
2. Explain why the ADR constraint is being relaxed
3. The user may choose to:
   - Accept the break and continue
   - Update the ADR to reflect the new decision

### After changes

1. Verify no ADR is broken, or document if one is
2. Test with `deno run --allow-all index.ts push --verbose`
3. If an ADR was broken, update the ADR's Consequences section to document the change