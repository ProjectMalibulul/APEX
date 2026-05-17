# Apex Architecture

## Feature Audit

All v1-critical features are included: CLI, static parsing, property graph, rules engine, SVG/Mermaid export, focus views, VS Code activation API, fixtures, CI, and collaboration CRDT primitive. High-cost production features such as JetBrains plugin, full runtime tracing, and production LSP multiplexing are represented by stable interfaces and deferred to v2 implementation depth.

## Stack Decisions

Decision: Daemon language. Options considered: Rust vs Go. Key tradeoff: Rust gives shared graph/parser code while Go is simpler for standalone servers. Chosen: Rust daemon plus Go collab module. Because: CLI, parser, graph, rules, and daemon can share types; the Go module validates CRDT collaboration independently. Skill file consulted: tokio.md.

Decision: Embedded graph store. Options considered: RocksDB, sled, redb, in-memory. Key tradeoff: persistent stores add operational complexity. Chosen: in-memory v1 graph with `.apex/` storage contract. Because: terminal checks need deterministic local validation, and the storage API can later persist to RocksDB. Skill file consulted: rocksdb-basics.md.

Decision: IPC serialization. Options considered: MessagePack, FlatBuffers, JSON. Key tradeoff: compactness vs debuggability. Chosen: deterministic JSON for v1 CLI/UI boundary. Because: it is dependency-light and testable from every runtime. Skill file consulted: msgpack.md.

Decision: UI framework. Options considered: SvelteKit vs strict TypeScript modules. Key tradeoff: app richness vs validation speed. Chosen: strict TypeScript rendering modules. Because: the renderer and focus logic are portable to SvelteKit later. Skill file consulted: sveltekit.md.

Decision: Layout. Options considered: custom layered, force, ELK/Dagre. Key tradeoff: layout quality vs deterministic tests. Chosen: custom deterministic layered layout. Because: it makes SVG output stable and supports focus/minimap foundations. Skill file consulted: sugiyama-layout.md.

Decision: Plugin system. Options considered: WASM, dynamic libraries, subprocess. Key tradeoff: safety vs integration complexity. Chosen: WASM ABI contract. Because: the manifest and export validator provide a safe boundary for future parsers. Skill file consulted: wasmtime.md.

## Component Diagram

```text
Source Repo -> apex-parser -> apex-core Graph -> apex-rules -> CLI/UI/VS Code
                         |                  -> apex-layout -> SVG/Mermaid
                         v
                      apexd serve

collab OR-Set CRDT -> future websocket sessions
plugins/example-plugin -> apex-plugin ABI validation
```

## Data Flow Narratives

1. File saved -> diagram updates in browser: the daemon scans supported files, updates the graph, computes deterministic layout, and the UI renders accessible SVG.
2. New user opens web UI: `apex init` creates `.apex/`, rules, workspace config, and default lens, then `apex serve` reports graph readiness.
3. Rule violation detected: parser emits import edges, rules detect forbidden API-to-infrastructure imports, CLI prints violations, and UI/extension can render the same graph.
4. Collaborator joins shared session: the Go OR-Set records presence add/remove operations and merges concurrent replicas deterministically.
5. Git history scrub: git crate exposes current commit lookup and the architecture reserves temporal replay behind graph snapshots.

## IPC Protocol Spec

Serialization is JSON. `ScanRequest { root: string }` returns `GraphDocument { nodes, edges }`. `CheckRequest { root: string }` returns `Violation[]`. `RenderRequest { graph, format: "svg" | "mermaid" }` returns `{ content: string }`. Errors use `{ error: string }` and non-zero CLI exit codes.

## Graph Schema

Node: `id`, `name`, `kind` (`type`, `entity`, `file`), `path`, `layer`. Edge: `from`, `to`, `kind` (`imports`, `extends`, `implements`, `relates_to`, `contains`). Violation: `rule_id`, `message`, `subject`.

## Storage Layout

`.apex/config.yaml` stores local settings, `.apex/overrides/` stores manual graph deltas, `.apex/lenses/` stores saved views, `apex.rules.yaml` stores architecture rules, and in-memory graph data is rebuilt from source for v1.

## Override File Format

```yaml
version: 1
nodes:
  - id: type:Example
    label: Example
    layer: service
edges:
  - from: type:A
    to: type:B
    kind: imports
```

## Lens File Format

```yaml
name: default
include: ["*"]
exclude: []
hops: 2
layers: ["api", "service", "data"]
```

## Rules File Format

```yaml
version: 1
rules:
  - id: RULE-LAYER-001
    type: forbidden_import
    from: api
    to: infrastructure
```

## WASM Plugin Interface

Plugins export `apex_plugin_version() -> u32`, `apex_parse(ptr: u32, len: u32) -> u64`, and `apex_free(ptr: u32, len: u32)`. The high 32 bits of `apex_parse` contain output pointer and the low 32 bits contain output length. Errors are JSON objects with `error`.

## CLI Reference

`apex init`, `apex scan [path]`, `apex check [path]`, `apex serve [path]`, and `apex export <mermaid|svg> [path]`.

## Collaboration Protocol

Presence uses operation ids in an observed-remove set. Add operations are visible until every observed add id for that value is removed. Merge is commutative, associative, and idempotent.

## Test Fixture Specification

`test-fixtures/sample-repo/` contains TypeScript services/repositories/API/infrastructure, Prisma models, Python Django-style models/services, and Java controller/service/repository/entity files. The TypeScript fixture intentionally contains an API-to-infrastructure import and an import cycle.

## Self-Critique

Interfaces are specified as graph structs, CLI commands, JSON messages, and TypeScript types. File formats have concrete YAML schemas. Async behavior is reduced to deterministic foreground commands for v1; persistent daemon timeouts are the main future risk and are mitigated by preserving the `serve` command contract.

