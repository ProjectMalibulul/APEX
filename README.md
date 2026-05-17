# Apex

Apex is a local-first architecture diagrammer for source repositories. It scans TypeScript, Python, Java, and Prisma fixtures into a property graph, checks architecture rules, renders SVG/Mermaid diagrams, exposes a CLI, and includes UI, VS Code extension, collaboration, and CI scaffolding.

## Quick start

```bash
cargo run -p apex-cli -- init
cargo run -p apex-cli -- scan test-fixtures/sample-repo
cargo run -p apex-cli -- check test-fixtures/sample-repo
cargo run -p apex-cli -- export mermaid test-fixtures/sample-repo
```

The `check` command intentionally reports the fixture's API-to-infrastructure rule violation and import cycle.

