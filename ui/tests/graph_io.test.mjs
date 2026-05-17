import assert from "node:assert/strict";
import test from "node:test";
import { execFileSync } from "node:child_process";
import { graphStats, parseGraphDocument } from "../../dist/ui/src/lib/graph_io.js";

test("parseGraphDocument accepts apex scan output", () => {
  const output = execFileSync("cargo", ["run", "-q", "-p", "apex-cli", "--", "scan", "test-fixtures/sample-repo"], {
    encoding: "utf8"
  });
  const graph = parseGraphDocument(output);
  const stats = graphStats(graph);
  assert.ok(stats.nodes > 10);
  assert.ok(stats.edges > 5);
  assert.ok(stats.layers.includes("api"));
});

test("parseGraphDocument rejects malformed graph json", () => {
  assert.throws(() => parseGraphDocument("{\"nodes\":[]}"), /nodes and edges/);
});

