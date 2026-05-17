import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import test from "node:test";
import { activate, parseCliGraph, renderClassDiagram, renderWebviewHtml } from "../../dist/vscode-extension/src/extension.js";

test("activate registers disposable and exposes renderer", () => {
  const context = { subscriptions: [] };
  const api = activate(context);
  assert.equal(context.subscriptions.length, 1);
  assert.equal(typeof api.renderClassDiagram, "function");
  assert.equal(typeof api.renderWebviewHtml, "function");
  assert.equal(typeof api.parseCliGraph, "function");
});

test("renderClassDiagram returns svg for class graph", () => {
  const svg = renderClassDiagram({
    nodes: [{ id: "type:UserService", name: "UserService", kind: "type", path: "UserService.ts", layer: "service" }],
    edges: []
  });
  assert.match(svg, /UserService/);
});

test("parseCliGraph consumes apex scan output and renders webview html", () => {
  const output = execFileSync("cargo", ["run", "-q", "-p", "apex-cli", "--", "scan", "test-fixtures/sample-repo"], {
    encoding: "utf8"
  });
  const graph = parseCliGraph(output);
  const html = renderWebviewHtml(graph);
  assert.match(html, /Apex Class Diagram/);
  assert.match(html, /UserService/);
  assert.ok(graph.nodes.length > 10);
});
