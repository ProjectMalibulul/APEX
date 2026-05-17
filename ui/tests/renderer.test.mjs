import assert from "node:assert/strict";
import test from "node:test";
import { renderSvg } from "../../dist/ui/src/lib/renderer/svg_renderer.js";
import { focusGraph } from "../../dist/ui/src/lib/layout.js";
import { toMermaid } from "../../dist/ui/src/lib/exporters.js";

const graph = {
  nodes: [
    { id: "type:UserService", name: "UserService", kind: "type", path: "UserService.ts", layer: "service" },
    { id: "type:UserRepository", name: "UserRepository", kind: "type", path: "UserRepository.ts", layer: "data" }
  ],
  edges: [{ from: "type:UserService", to: "type:UserRepository", kind: "imports" }]
};

test("renderSvg returns accessible svg", () => {
  const svg = renderSvg(graph);
  assert.match(svg, /role="img"/);
  assert.match(svg, /UserService/);
});

test("focusGraph returns one hop neighbours", () => {
  const focused = focusGraph(graph, "type:UserService", 1);
  assert.equal(focused.nodes.length, 2);
  assert.equal(focused.edges.length, 1);
});

test("toMermaid returns class diagram", () => {
  const mermaid = toMermaid(graph);
  assert.match(mermaid, /classDiagram/);
  assert.match(mermaid, /UserService/);
});

