## ADR Reading Guide

When reading an ADR in `docs/adrs/`, follow these guidelines:

### Cross-Referencing

- ADRs often reference each other (e.g., `[[ADR-001]]`, `[[ADR-005]]`)
- Always check referenced ADRs to understand the full context
- Decisions in one ADR may depend on or override decisions in another

### Related ADRs

- ADR-001 (Issue File Matching) → ADR-002 (Title Resolution) → ADR-004 (Idempotency)
- ADR-005 (Folder Structure) → ADR-006 (Clean Architecture)
- When making changes, check if related ADRs need updating

### ADR Format

Each ADR has:
- **Context**: Why the decision was needed
- **Decision**: What was decided
- **Consequences**: Positive and negative outcomes
- **References**: Related ADRs and external links