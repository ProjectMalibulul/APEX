# Apex

Apex is a local-first architecture workbench for understanding source repositories. It scans code, builds an architecture graph, checks rules, and generates diagrams from your machine without uploading source anywhere.

## Why use Apex?

- **See the architecture quickly**: generate SVG, Mermaid, HTML, and JSON diagrams from a repository path.
- **Check architecture rules locally**: catch forbidden layer imports and dependency cycles before review.
- **Compute architecture metrics**: hotspots (fan-in/fan-out), import cycles, components, layer mix, orphans via `apex metrics`.
- **Use a real local UI**: `apex ui` serves a Vite-built workbench with pan/zoom canvas, scan/check/render/metrics actions, and per-class member compartments in the SVG.
- **Broad language support**: TypeScript/JavaScript, Python, Java, Go, Rust, Kotlin, C#, C/C++, Swift, PHP, Ruby, Scala, Dart, Prisma, SQL, plus manifest dependency edges (`package.json`, `Cargo.toml`).
- **Automate everything**: `just verify` runs formatting, builds, tests, lints, diagram generation, CLI smoke checks, UI server/API smoke checks (incl. `/api/metrics`), and VS Code integration smoke checks.
- **Cross-platform releases**: tag a `v*` and the `release` GitHub Action builds binaries for Linux x86_64/aarch64, macOS x86_64/aarch64, and Windows x86_64.

## Quick start

```bash
just setup
just verify
just ui
```

Open `http://127.0.0.1:4317/`, keep the default `test-fixtures/sample-repo` path, and click:

1. **Scan repository** to build the graph.
2. **Run rules** to see violations.
3. **Render via API** to generate SVG, Mermaid, HTML, or JSON.
4. **Download current output** to save the diagram locally.

## CLI examples

```bash
cargo run -p apex-cli -- languages
cargo run -p apex-cli -- capabilities
cargo run -p apex-cli -- init
cargo run -p apex-cli -- scan test-fixtures/sample-repo
cargo run -p apex-cli -- check test-fixtures/sample-repo
cargo run -p apex-cli -- rules list
cargo run -p apex-cli -- rules template --out apex.rules.yaml
cargo run -p apex-cli -- diagram test-fixtures/sample-repo --format svg --out artifacts/apex.svg
cargo run -p apex-cli -- metrics test-fixtures/sample-repo
cargo run -p apex-cli -- metrics test-fixtures/sample-repo --format json
cargo run -p apex-cli -- ui
```

## Task shortcuts

```bash
just setup          # install npm dependencies
just build          # Rust workspace + TypeScript + Vite UI
just test           # Rust, TypeScript, VS Code module tests, Go tests
just lint           # clippy, tsc --strict, go vet
just diagram        # generate SVG, Mermaid, HTML, and JSON locally
just metrics        # print architecture metrics for the sample repo
just ui             # build and launch the Apex UI server
just ui-dev         # Vite dev server with /api proxy to apex ui
just ui-smoke       # UI server/API/built-app smoke test (includes /api/metrics)
just vscode-smoke   # VS Code extension integration smoke test
just release-local  # produce a release-style bundle in artifacts/local-release
just verify         # full local verification
```

## Releases

Tag-triggered cross-platform builds live in `.github/workflows/release.yml`. Push `vX.Y.Z` and the workflow produces:

- `apex-vX.Y.Z-linux-x86_64.tar.gz`
- `apex-vX.Y.Z-linux-aarch64.tar.gz`
- `apex-vX.Y.Z-macos-x86_64.tar.gz`
- `apex-vX.Y.Z-macos-aarch64.tar.gz`
- `apex-vX.Y.Z-windows-x86_64.zip`

Each archive contains the `apex` binary, the built `ui-dist/`, README, LICENSE, and `docs/`.

## Documentation

User-facing docs live in [`docs/`](docs/):

- [Getting started](docs/getting-started.md)
- [CLI reference](docs/cli.md)
- [Local UI guide](docs/ui.md)
- [Rules guide](docs/rules.md)
- [Language support](docs/languages.md)
- [Diagram generation](docs/diagrams.md)
- [Metrics](docs/metrics.md)
- [Configuration](docs/configuration.md)
- [VS Code integration](docs/vscode.md)
- [Troubleshooting](docs/troubleshooting.md)

Internal build notes and generated state belong in `.state/`, which is ignored by Git.

