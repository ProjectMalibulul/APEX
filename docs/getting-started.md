# Getting started

## Prerequisites

- Rust with `cargo`, `rustfmt`, and `clippy`
- Node.js and npm
- Go
- `just`

## Install dependencies

```bash
just setup
```

## Verify the whole workspace

```bash
just verify
```

This runs Rust, TypeScript, Vite, Go, CLI, UI server/API, diagram, and VS Code smoke checks.

## Start the local workbench

```bash
just ui
```

Open `http://127.0.0.1:4317/`.

## Generate diagrams without the UI

```bash
just diagram test-fixtures/sample-repo
```

Outputs are written to `artifacts/`.

