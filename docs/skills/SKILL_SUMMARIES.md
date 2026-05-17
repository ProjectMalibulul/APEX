# Skill Summaries

## rust-style.md
- Rust formatting and linting are enforced through `cargo fmt` and `cargo clippy`.
- Public APIs should be documented and small modules should expose clear contracts.
- Apex applies this to all workspace crates.

## tree-sitter.md / tree-sitter-using.md
- Parser adapters should emit language-neutral graph nodes and edges.
- Concrete syntax details should remain behind parser modules.
- Apex v1 uses lightweight recognizers but keeps this adapter boundary.

## rocksdb-basics.md
- Embedded storage should define clear key spaces and recovery behavior.
- Apex v1 documents `.apex/` storage and keeps graph persistence replaceable.
- Future RocksDB integration should not change CLI output contracts.

## tokio.md / axum.md
- Async services need explicit startup, cancellation, and error boundaries.
- Apex v1 keeps `serve` deterministic and foreground for validation.
- Future persistent transports will live behind the existing daemon contract.

## sveltekit.md / tsconfig.md
- TypeScript should run in strict mode with typed graph documents.
- UI modules should be framework-portable and avoid `any`.
- Apex uses strict TypeScript modules for renderer, layout, and exporters.

## vscode-extension.md
- Extensions activate through declared activation events and return disposable subscriptions.
- Apex exposes a renderer API during activation for testable class diagrams.
- Runtime VS Code integration can wrap the same pure module later.

## wasmtime.md
- WASM plugin boundaries require explicit exports, memory ownership, and error contracts.
- Apex validates required export names and documents parse/free conventions.
- Plugin code cannot assume host memory ownership.

## github-api.md / git-internals.md
- Repository publication uses authenticated GitHub CLI/API calls.
- Git metadata is optional when not inside a repository.
- Apex keeps git failures non-fatal for local scans.

## crdt.md / effective-go.md
- CRDT merges should be deterministic, idempotent, and explicit about operation ids.
- Go code should return deterministic values and be vet-clean.
- Apex implements an observed-remove set for collaboration presence.

## sugiyama-layout.md / svg-accessibility.md
- Layered diagrams should be deterministic and readable for hierarchical graphs.
- SVG diagrams should expose accessible roles, labels, and keyboard focus.
- Apex renders stable layered SVG nodes and edges.

## lsp-spec.md / msgpack.md / prisma-schema.md
- LSP enrichment and MessagePack transport are documented extension points.
- Prisma schemas are parsed into entity nodes and relation edges.
- Apex v1 preserves JSON CLI output for debuggability.

