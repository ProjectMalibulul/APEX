# Local Apex Workflow

## Generate diagrams locally

Use the CLI directly or through `just diagram`:

```bash
just diagram test-fixtures/sample-repo
```

Generated files:

| File | Description |
|---|---|
| `artifacts/apex.svg` | Standalone accessible SVG diagram |
| `artifacts/apex.mmd` | Mermaid class diagram |
| `artifacts/apex.html` | Self-contained browser viewer with embedded SVG and JSON |
| `artifacts/apex.json` | Raw graph document from the parser |

`artifacts/` is gitignored because it is regenerated from source.

## Run the dedicated UI

```bash
just ui
```

Open `http://127.0.0.1:4317/`. The UI is served by the Apex CLI and has local API routes:

| Route | Description |
|---|---|
| `/api/health` | UI server readiness |
| `/api/scan?path=<repo>` | Returns graph JSON |
| `/api/check?path=<repo>` | Returns rule violations |
| `/api/diagram?path=<repo>&format=svg` | Returns generated SVG |
| `/api/diagram?path=<repo>&format=mermaid` | Returns Mermaid |
| `/api/diagram?path=<repo>&format=html` | Returns self-contained HTML |
| `/api/diagram?path=<repo>&format=json` | Returns graph JSON |

The browser workbench can scan a path, run rules, preview SVG, render API output in multiple formats, import graph JSON, apply focus mode, and download the current output.

For frontend development, keep `apex ui` running and start Vite:

```bash
npm run dev:ui
```

Vite serves the frontend at `http://127.0.0.1:5173/` and proxies `/api` to the Apex UI server.

## Test VS Code integration without launching VS Code

```bash
just vscode-smoke
```

This validates the extension's integration boundary by feeding real CLI graph JSON into the extension module and rendering the webview HTML body.

## Full local verification

```bash
just verify
```

This runs formatting, Rust tests, TypeScript strict build and tests, Vite build, Go tests, clippy, go vet, diagram generation, CLI smoke checks, dedicated UI server/API smoke checks, and VS Code smoke tests.
