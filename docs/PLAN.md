# Apex v1 Plan

Implement a compact local-first monorepo that validates the required end-to-end flows: parse sample repositories, store graph data in memory, detect rule violations, render diagrams, expose CLI commands, compile TypeScript UI and VS Code extension modules, provide a Go CRDT collaboration primitive, and publish the repository.

The design intentionally keeps runtime dependencies low for deterministic builds. Future production hardening can replace lightweight recognizers with tree-sitter/LSP adapters without changing the graph, CLI, renderer, or rule contracts.

