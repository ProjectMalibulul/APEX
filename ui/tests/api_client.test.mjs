import assert from "node:assert/strict";
import test from "node:test";
import { buildApiUrl } from "../../dist/ui/src/lib/api_client.js";

test("buildApiUrl encodes repository paths and formats", () => {
  const url = buildApiUrl("diagram", { path: "fixtures/sample repo", format: "svg" });
  assert.equal(url, "/api/diagram?path=fixtures%2Fsample+repo&format=svg");
});

test("built ui contains rule and language management actions", async () => {
  const html = await import("node:fs/promises").then((fs) => fs.readFile("ui/dist/index.html", "utf8"));
  assert.match(html, /Apex Workbench/);
});
