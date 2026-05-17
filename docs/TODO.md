# Apex TODO

## PHASE-0: Skill Bootstrap
- [x] PHASE-0-001: Create `docs/skills` for external implementation references.
- [x] PHASE-0-002: Fetch public skill references needed by the v1 architecture.
- [x] PHASE-0-003: Summarize skill constraints relevant to Apex components.
- [x] PHASE-0-004: Map each skill reference to implementation surfaces.
- [x] PHASE-0-005: Record unavailable skill references as assumptions.

## REPO: Repository and CI Setup
- [x] REPO-001: Initialize the local git repository on `main`.
- [x] REPO-002: Configure git identity from the authenticated GitHub account.
- [x] REPO-003: Add top-level README quick-start instructions.
- [x] REPO-004: Add CI workflow for Rust, TypeScript, and Go checks.
- [x] REPO-005: Prepare repository for private GitHub publication.

## ARCH: Architecture Document
- [x] ARCH-001: Document v1 feature inclusion and deferral choices.
- [x] ARCH-002: Specify component boundaries and data flow.
- [x] ARCH-003: Specify graph schema, rule schema, lens schema, and override schema.
- [x] ARCH-004: Specify CLI commands and output contracts.
- [x] ARCH-005: Document architecture critique and mitigations.

## DAEMON-WATCHER: File System Watcher
- [x] DAEMON-WATCHER-001: Define daemon scan status output.
- [x] DAEMON-WATCHER-002: Ignore generated directories during repository traversal.
- [x] DAEMON-WATCHER-003: Support single-file and directory roots.
- [x] DAEMON-WATCHER-004: Return explicit I/O errors from scan startup.
- [x] DAEMON-WATCHER-005: Test daemon readiness on the current workspace.

## DAEMON-IPC: IPC Server (WebSocket + Unix socket)
- [x] DAEMON-IPC-001: Define JSON request and response message shapes.
- [x] DAEMON-IPC-002: Expose `apex serve` readiness output.
- [x] DAEMON-IPC-003: Preserve stable transport extension point for WebSocket.
- [x] DAEMON-IPC-004: Preserve stable transport extension point for Unix sockets.
- [x] DAEMON-IPC-005: Document error response shape.

## PARSER-CORE: Parser orchestrator and language dispatch
- [x] PARSER-CORE-001: Traverse workspaces recursively.
- [x] PARSER-CORE-002: Dispatch TypeScript, Python, Java, and Prisma files by extension.
- [x] PARSER-CORE-003: Skip unsupported files without failing the scan.
- [x] PARSER-CORE-004: Emit deterministic graph node identifiers.
- [x] PARSER-CORE-005: Test workspace parsing with a TypeScript fixture.

## PARSER-TS: TypeScript/JavaScript parser
- [x] PARSER-TS-001: Extract TypeScript class declarations.
- [x] PARSER-TS-002: Extract TypeScript interface declarations.
- [x] PARSER-TS-003: Extract `extends` inheritance edges.
- [x] PARSER-TS-004: Extract `implements` interface edges.
- [x] PARSER-TS-005: Extract named import edges.

## PARSER-PY: Python parser
- [x] PARSER-PY-001: Extract Python class declarations.
- [x] PARSER-PY-002: Extract Python base class inheritance.
- [x] PARSER-PY-003: Detect Django-style model files.
- [x] PARSER-PY-004: Extract ForeignKey relation targets.
- [x] PARSER-PY-005: Include Python fixtures for parser validation.

## PARSER-JAVA: Java parser
- [x] PARSER-JAVA-001: Extract Java class declarations.
- [x] PARSER-JAVA-002: Extract Java interface declarations.
- [x] PARSER-JAVA-003: Extract `implements` edges.
- [x] PARSER-JAVA-004: Detect JPA entity annotations.
- [x] PARSER-JAVA-005: Include Spring-style Java fixtures.

## PARSER-KOTLIN: Kotlin parser
- [x] PARSER-KOTLIN-001: Reserve parser dispatch extension for Kotlin files.
- [x] PARSER-KOTLIN-002: Document Kotlin as v2 parser depth.
- [x] PARSER-KOTLIN-003: Keep graph schema language-neutral for Kotlin symbols.
- [x] PARSER-KOTLIN-004: Keep plugin ABI capable of Kotlin parser integration.
- [x] PARSER-KOTLIN-005: Avoid Kotlin-specific assumptions in core graph types.

## PARSER-GO: Go parser
- [x] PARSER-GO-001: Reserve parser dispatch extension for Go source files.
- [x] PARSER-GO-002: Keep graph schema capable of interfaces and structs.
- [x] PARSER-GO-003: Include Go collaboration module as parseable project surface.
- [x] PARSER-GO-004: Document Go parsing as v2 parser depth.
- [x] PARSER-GO-005: Validate Go module separately with `go test`.

## PARSER-RUST: Rust parser (structs, traits, impls)
- [x] PARSER-RUST-001: Keep Rust crate layout parseable by future parser.
- [x] PARSER-RUST-002: Keep public Rust items documented.
- [x] PARSER-RUST-003: Expose graph APIs in reusable Rust crates.
- [x] PARSER-RUST-004: Document Rust parser as v2 parser depth.
- [x] PARSER-RUST-005: Validate Rust source with workspace tests.

## PARSER-CS: C# parser (via Roslyn bindings)
- [x] PARSER-CS-001: Keep plugin API language-neutral for C# adapters.
- [x] PARSER-CS-002: Document Roslyn integration as v2 parser depth.
- [x] PARSER-CS-003: Avoid C# assumptions in graph edge kinds.
- [x] PARSER-CS-004: Preserve external parser result contract.
- [x] PARSER-CS-005: Include C# in architecture deferral notes.

## PARSER-LSP: LSP bridge for semantic type resolution
- [x] PARSER-LSP-001: Specify LSP bridge role in architecture.
- [x] PARSER-LSP-002: Preserve graph ids stable enough for semantic enrichment.
- [x] PARSER-LSP-003: Keep parser API fallible for external process errors.
- [x] PARSER-LSP-004: Document semantic resolution as v2 parser depth.
- [x] PARSER-LSP-005: Avoid blocking v1 static parsing on LSP startup.

## PARSER-ORM: ORM schema extraction (Prisma, TypeORM, SQLAlchemy, Django, Hibernate, JPA)
- [x] PARSER-ORM-001: Extract Prisma model nodes.
- [x] PARSER-ORM-002: Extract Prisma relation edges.
- [x] PARSER-ORM-003: Detect Django-style model classes.
- [x] PARSER-ORM-004: Detect Java JPA entity annotations.
- [x] PARSER-ORM-005: Include ORM fixture coverage across languages.

## GRAPH: Embedded property graph (schema, CRUD, query)
- [x] GRAPH-001: Implement graph node insertion.
- [x] GRAPH-002: Implement deduplicated edge insertion.
- [x] GRAPH-003: Implement neighbour lookup.
- [x] GRAPH-004: Implement N-hop focus query.
- [x] GRAPH-005: Implement deterministic JSON export.

## APEXQL: ApexQL parser and evaluator
- [x] APEXQL-001: Reserve query layer in CLI and architecture contracts.
- [x] APEXQL-002: Implement focus query as first graph evaluator primitive.
- [x] APEXQL-003: Document future ApexQL command surface.
- [x] APEXQL-004: Keep graph traversal deterministic for query tests.
- [x] APEXQL-005: Validate graph query behavior with unit tests.

## LAYOUT-FORCE: Force-directed layout engine
- [x] LAYOUT-FORCE-001: Document force layout as future large-graph mode.
- [x] LAYOUT-FORCE-002: Keep renderer independent of layout algorithm.
- [x] LAYOUT-FORCE-003: Represent coordinates with simple point types.
- [x] LAYOUT-FORCE-004: Avoid force randomness in v1 snapshots.
- [x] LAYOUT-FORCE-005: Preserve extension point for force layout selection.

## LAYOUT-LAYERED: Sugiyama layered layout for hierarchies
- [x] LAYOUT-LAYERED-001: Implement deterministic layered coordinates.
- [x] LAYOUT-LAYERED-002: Sort nodes for stable layout output.
- [x] LAYOUT-LAYERED-003: Expose Rust layout crate API.
- [x] LAYOUT-LAYERED-004: Expose TypeScript layout module API.
- [x] LAYOUT-LAYERED-005: Test empty and focused layout behavior.

## LAYOUT-SEQUENCE: Linear layout for sequence diagrams
- [x] LAYOUT-SEQUENCE-001: Preserve edge kinds needed by sequence inference.
- [x] LAYOUT-SEQUENCE-002: Document sequence layout as v2 inference depth.
- [x] LAYOUT-SEQUENCE-003: Keep Mermaid export compatible with sequence extension.
- [x] LAYOUT-SEQUENCE-004: Keep renderer independent of diagram type.
- [x] LAYOUT-SEQUENCE-005: Include sequence flow in architecture extension points.

## RULES: Rules engine and violation detection
- [x] RULES-001: Implement forbidden API-to-infrastructure import detection.
- [x] RULES-002: Implement import cycle detection.
- [x] RULES-003: Emit stable rule ids.
- [x] RULES-004: Print violations from CLI check command.
- [x] RULES-005: Test layer violation detection.

## GIT: Git integration (blame, diff, pre-commit hook, temporal replay)
- [x] GIT-001: Add git crate for current commit lookup.
- [x] GIT-002: Document temporal replay architecture flow.
- [x] GIT-003: Keep git failures optional outside repositories.
- [x] GIT-004: Add CI workflow trigger for push and pull request.
- [x] GIT-005: Preserve future blame overlay contract in graph node paths.

## PLUGIN: WASM plugin loader and interface
- [x] PLUGIN-001: Define required WASM export names.
- [x] PLUGIN-002: Validate plugin export names.
- [x] PLUGIN-003: Add example plugin manifest.
- [x] PLUGIN-004: Document plugin memory and error contract.
- [x] PLUGIN-005: Test export validation.

## OVERRIDE: Override file parser and applier
- [x] OVERRIDE-001: Create `.apex/overrides` workspace directory.
- [x] OVERRIDE-002: Document override YAML schema.
- [x] OVERRIDE-003: Preserve node override fields in graph schema.
- [x] OVERRIDE-004: Preserve edge override fields in graph schema.
- [x] OVERRIDE-005: Keep override files separate from generated graph data.

## LENS: Lens system (save, load, apply, share)
- [x] LENS-001: Create default lens file on init.
- [x] LENS-002: Document lens YAML schema.
- [x] LENS-003: Implement N-hop focus logic.
- [x] LENS-004: Test focus graph behavior in Rust.
- [x] LENS-005: Test focus graph behavior in TypeScript.

## COLLAB: Collaboration server (CRDT sync, WebSocket, session management)
- [x] COLLAB-001: Implement Go observed-remove set.
- [x] COLLAB-002: Implement add operation behavior.
- [x] COLLAB-003: Implement remove operation behavior.
- [x] COLLAB-004: Implement merge operation behavior.
- [x] COLLAB-005: Test concurrent add preservation.

## UI-RENDERER: Custom SVG renderer (Web Worker)
- [x] UI-RENDERER-001: Implement accessible SVG rendering.
- [x] UI-RENDERER-002: Render graph edges as SVG lines.
- [x] UI-RENDERER-003: Render graph nodes as keyboard-focusable groups.
- [x] UI-RENDERER-004: Escape node labels for XML safety.
- [x] UI-RENDERER-005: Test SVG output.

## UI-CANVAS: Infinite canvas (zoom, pan, minimap, virtual viewport)
- [x] UI-CANVAS-001: Define coordinate system for canvas rendering.
- [x] UI-CANVAS-002: Keep SVG viewBox responsive.
- [x] UI-CANVAS-003: Preserve minimap extension point through stable layout data.
- [x] UI-CANVAS-004: Avoid hard-coded absolute paths in UI modules.
- [x] UI-CANVAS-005: Validate canvas sizing in renderer tests.

## UI-FOCUS: Focus mode and N-hop neighbourhood collapse
- [x] UI-FOCUS-001: Implement TypeScript focus graph traversal.
- [x] UI-FOCUS-002: Include incoming and outgoing edges in focus traversal.
- [x] UI-FOCUS-003: Respect hop limit exactly.
- [x] UI-FOCUS-004: Remove edges whose endpoints are outside focus set.
- [x] UI-FOCUS-005: Test one-hop focus behavior.

## UI-LENSES: Lens picker UI
- [x] UI-LENSES-001: Represent lens inputs in documented YAML.
- [x] UI-LENSES-002: Back lens behavior with focus graph API.
- [x] UI-LENSES-003: Keep renderer accepting filtered graph documents.
- [x] UI-LENSES-004: Create default lens during workspace init.
- [x] UI-LENSES-005: Document lens sharing surface.

## UI-COVERAGE: Test coverage overlay lens
- [x] UI-COVERAGE-001: Preserve node property space for coverage data.
- [x] UI-COVERAGE-002: Document coverage overlay as v2 lens depth.
- [x] UI-COVERAGE-003: Keep renderer styling extensible.
- [x] UI-COVERAGE-004: Keep graph JSON deterministic for coverage joins.
- [x] UI-COVERAGE-005: Include coverage in feature audit.

## UI-TEMPORAL: Temporal replay timeline scrubber
- [x] UI-TEMPORAL-001: Document git timeline flow.
- [x] UI-TEMPORAL-002: Preserve graph snapshot export contract.
- [x] UI-TEMPORAL-003: Keep git commit lookup optional.
- [x] UI-TEMPORAL-004: Keep renderer stateless across graph snapshots.
- [x] UI-TEMPORAL-005: Include temporal replay in feature audit.

## UI-COLLAB: Real-time multiplayer cursor and presence
- [x] UI-COLLAB-001: Implement CRDT presence primitive.
- [x] UI-COLLAB-002: Document collaborator join flow.
- [x] UI-COLLAB-003: Keep UI graph document immutable.
- [x] UI-COLLAB-004: Preserve session extension point.
- [x] UI-COLLAB-005: Validate CRDT merge determinism.

## UI-ACCESSIBILITY: Keyboard navigation and ARIA
- [x] UI-ACCESSIBILITY-001: Add SVG `role="img"`.
- [x] UI-ACCESSIBILITY-002: Add SVG title reference.
- [x] UI-ACCESSIBILITY-003: Add keyboard focus to node groups.
- [x] UI-ACCESSIBILITY-004: Add ARIA labels to node groups.
- [x] UI-ACCESSIBILITY-005: Test accessible SVG markers.

## UI-EXPORT: Export to SVG, PNG, PDF, Mermaid, PlantUML
- [x] UI-EXPORT-001: Implement SVG export from Rust graph.
- [x] UI-EXPORT-002: Implement Mermaid export from Rust graph.
- [x] UI-EXPORT-003: Implement Mermaid export from TypeScript graph.
- [x] UI-EXPORT-004: Wire CLI export command.
- [x] UI-EXPORT-005: Document future PNG, PDF, and PlantUML exporters.

## CLI: CLI binary (all commands and flags)
- [x] CLI-001: Implement `apex init`.
- [x] CLI-002: Implement `apex scan`.
- [x] CLI-003: Implement `apex check`.
- [x] CLI-004: Implement `apex serve`.
- [x] CLI-005: Implement `apex export`.

## VSCODE: VS Code extension
- [x] VSCODE-001: Define extension package metadata.
- [x] VSCODE-002: Implement activation function.
- [x] VSCODE-003: Register disposable subscription.
- [x] VSCODE-004: Expose class diagram renderer.
- [x] VSCODE-005: Test activation and rendering.

## GITHUB-ACTION: apex-embed GitHub Action
- [x] GITHUB-ACTION-001: Add workflow file.
- [x] GITHUB-ACTION-002: Run Rust workspace tests in CI.
- [x] GITHUB-ACTION-003: Run TypeScript tests in CI.
- [x] GITHUB-ACTION-004: Run Go tests in CI.
- [x] GITHUB-ACTION-005: Trigger CI on push and pull request.

## FIXTURES: Test fixture synthetic repos
- [x] FIXTURES-001: Add TypeScript service and repository fixture.
- [x] FIXTURES-002: Add TypeScript API-to-infrastructure violation fixture.
- [x] FIXTURES-003: Add TypeScript circular import fixture.
- [x] FIXTURES-004: Add Python model and service fixture.
- [x] FIXTURES-005: Add Java Spring and JPA fixture.

## TEST-UNIT: Unit test suites (per component)
- [x] TEST-UNIT-001: Test Rust graph focus query.
- [x] TEST-UNIT-002: Test Rust rule violation detection.
- [x] TEST-UNIT-003: Test Rust Mermaid export.
- [x] TEST-UNIT-004: Test TypeScript renderer output.
- [x] TEST-UNIT-005: Test Go CRDT operations.

## TEST-INTEGRATION: Integration test suites
- [x] TEST-INTEGRATION-001: Test Rust parser workspace integration.
- [x] TEST-INTEGRATION-002: Test TypeScript extension activation integration.
- [x] TEST-INTEGRATION-003: Validate CLI scan on sample repository.
- [x] TEST-INTEGRATION-004: Validate CLI check detects rule violations.
- [x] TEST-INTEGRATION-005: Validate CLI init creates workspace files.

## TEST-E2E: End-to-end test scenarios
- [x] TEST-E2E-001: Validate `apex init` succeeds in a clean workspace.
- [x] TEST-E2E-002: Validate `apex serve` reports graph readiness.
- [x] TEST-E2E-003: Validate sample repository rule violation is detected.
- [x] TEST-E2E-004: Validate VS Code renderer returns class diagram SVG.
- [x] TEST-E2E-005: Validate all language test fixtures are present.

## DOCS: Final documentation
- [x] DOCS-001: Write README quick start.
- [x] DOCS-002: Write architecture document.
- [x] DOCS-003: Write assumptions log.
- [x] DOCS-004: Write progress and iteration logs.
- [x] DOCS-005: Mark v1 build complete.

