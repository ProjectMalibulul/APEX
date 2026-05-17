# Diagram generation

Generate diagrams locally:

```bash
cargo run -p apex-cli -- diagram . --format svg --out artifacts/apex.svg
cargo run -p apex-cli -- diagram . --format mermaid --out artifacts/apex.mmd
cargo run -p apex-cli -- diagram . --format html --out artifacts/apex.html
cargo run -p apex-cli -- diagram . --format json --out artifacts/apex.json
```

## Formats

| Format | Use when |
|---|---|
| SVG | You want an accessible standalone image |
| Mermaid | You want to paste diagrams into Markdown tools |
| HTML | You want a browser-viewable file with embedded graph data |
| JSON | You want to integrate Apex graph data with other tools |

`just diagram` generates all four formats for the sample repository.

