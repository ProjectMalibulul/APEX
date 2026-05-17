# Apex

Apex is a local-first architecture diagramming toolkit. It scans source repositories, builds a property graph, detects architecture rule violations, and generates diagrams without sending source code to a remote service.

## What works locally

- **CLI**: initialize workspaces, scan repos, check rules, serve readiness, and generate SVG, Mermaid, HTML, or JSON diagrams.
- **Graph engine**: deterministic nodes and edges for files, classes, interfaces, inheritance, imports, ORM entities, and relations.
- **Rules engine**: forbidden layer imports and import-cycle detection.
- **UI**: a dedicated local web workbench served by `apex ui`, backed by Apex API routes for scan, check, and diagram generation.
- **VS Code integration surface**: extension activation, CLI graph parsing, SVG rendering, and webview HTML rendering are tested.
- **Collaboration primitive**: Go observed-remove set CRDT for deterministic presence merging.
- **Fixtures**: TypeScript, Prisma, Python, and Java sample repositories with intentional violations.
- **Task runner**: `just` recipes for setup, build, lint, test, diagram generation, smoke tests, and full verification.

## Requirements

- Rust with `cargo`, `rustfmt`, and `clippy`
- Node.js and npm
- Go
- `just`

The repo pins Rust tooling in `rust-toolchain.toml`.

## Setup

```bash
just setup
just verify
```

`just verify` formats, builds, tests, lints, generates diagrams into `artifacts/`, and runs CLI + VS Code smoke checks.

## CLI usage

```bash
cargo run -p apex-cli -- init
cargo run -p apex-cli -- scan test-fixtures/sample-repo
cargo run -p apex-cli -- check test-fixtures/sample-repo
cargo run -p apex-cli -- diagram test-fixtures/sample-repo --format svg --out artifacts/apex.svg
cargo run -p apex-cli -- diagram test-fixtures/sample-repo --format mermaid --out artifacts/apex.mmd
cargo run -p apex-cli -- diagram test-fixtures/sample-repo --format html --out artifacts/apex.html
```

The sample repository intentionally fails `apex check` because `TokenValidator` imports infrastructure from the API layer and the TypeScript fixture contains a circular import.

## Just recipes

```bash
just setup          # install npm dependencies
just fmt            # cargo fmt + gofmt
just build          # Rust workspace + TypeScript
just test           # Rust, TypeScript, VS Code module tests, Go tests
just lint           # clippy, tsc --strict, go vet
just diagram        # generate SVG, Mermaid, HTML, and JSON diagrams locally
just ui             # build and launch the dedicated Apex UI at http://127.0.0.1:4317
just ui-dev         # run the Vite frontend with /api proxy to apex ui
just ui-smoke       # test the Apex UI server, API routes, and built app
just vscode-smoke   # test extension activation and CLI graph rendering
just smoke          # CLI init/serve/scan/check plus VS Code smoke
just verify         # full local verification
```

## Local UI

Run the dedicated local UI server:

```bash
just ui
```

Open `http://127.0.0.1:4317/`. The workbench can scan a repository path, run rules, render SVG/Mermaid/HTML/JSON through local API endpoints, import graph JSON files, focus on nodes, and download generated output.

The UI is built by Vite and served by the Apex CLI. It is not a generic static file server.

For frontend development with hot reload:

```bash
cargo run -p apex-cli -- ui
npm run dev:ui
```

Open `http://127.0.0.1:5173/`; Vite proxies `/api` to `apex ui`.

## VS Code integration smoke

The extension module is tested without requiring VS Code to launch. The smoke test:

1. Builds the CLI.
2. Runs `apex scan test-fixtures/sample-repo`.
3. Parses the CLI graph JSON through the extension.
4. Renders webview HTML containing the local SVG diagram.

Run it with:

```bash
just vscode-smoke
```

## Architecture

The workspace is split by responsibility:

| Path | Purpose |
|---|---|
| `crates/apex-core` | graph types, parser recognizers, rules, SVG/Mermaid export |
| `crates/apex-cli` | command-line interface and local diagram generation |
| `crates/apexd` | daemon readiness scan surface |
| `crates/apex-layout` | deterministic layout API |
| `crates/apex-parser` | parser orchestration API |
| `crates/apex-rules` | rules API wrapper |
| `ui/` | Vite workbench, API-backed UI interactions, renderer modules |
| `vscode-extension/` | VS Code activation/rendering integration surface |
| `collab/` | Go CRDT collaboration primitive |
| `test-fixtures/` | source repositories used by integration tests |

More detail is in `docs/ARCHITECTURE.md` and `docs/LOCAL_WORKFLOW.md`.
