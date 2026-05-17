# Assumptions Log

| ID | Component | Assumption | Rationale | Impact if Wrong |
|---|---|---|---|---|
| A-001 | Repository | Build locally in the current `APEX` directory, then publish to `PrakyathPNayak/apex-uml`. | User selected local build plus GitHub creation/push. | Remote naming would need adjustment. |
| A-002 | Daemon | `apex serve` performs a deterministic foreground readiness scan and exits successfully. | This makes CLI validation non-interactive in a terminal environment. | A persistent daemon can be added behind the same command contract. |
| A-003 | Parser | Lightweight static recognizers are acceptable for v1 fixtures. | They cover the required fixture languages without external parser downloads. | Production language precision will require tree-sitter/LSP follow-up. |

| SKILL-1779022131037436141 | skills | Skill file `wasmtime.md` unavailable, proceeding with implementation knowledge. | Public source fetch failed. | Future implementation may need manual review. |
