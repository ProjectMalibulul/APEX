import assert from "node:assert/strict";
import test from "node:test";
import { activate, renderClassDiagram } from "../../dist/vscode-extension/src/extension.js";

test("activate registers disposable and exposes renderer", () => {
  const context = { subscriptions: [] };
  const api = activate(context);
  assert.equal(context.subscriptions.length, 1);
  assert.equal(typeof api.renderClassDiagram, "function");
});

test("renderClassDiagram returns svg for class graph", () => {
  const svg = renderClassDiagram({
    nodes: [{ id: "type:UserService", name: "UserService", kind: "type", path: "UserService.ts", layer: "service" }],
    edges: []
  });
  assert.match(svg, /UserService/);
});

