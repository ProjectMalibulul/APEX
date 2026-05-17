import {
  checkRepository,
  listLanguages,
  listRules,
  renderDiagram,
  scanRepository,
  type DiagramFormat
} from "./lib/api_client.js";
import { toMermaid } from "./lib/exporters.js";
import { graphStats, parseGraphDocument } from "./lib/graph_io.js";
import { focusGraph } from "./lib/layout.js";
import { renderSvg } from "./lib/renderer/svg_renderer.js";
import type { GraphDocument } from "./lib/types.js";
import { sampleGraph } from "./sample.js";

const repoPath = requireElement<HTMLInputElement>("#repo-path");
const scanButton = requireElement<HTMLButtonElement>("#scan-button");
const checkButton = requireElement<HTMLButtonElement>("#check-button");
const loadRulesButton = requireElement<HTMLButtonElement>("#load-rules-button");
const loadLanguagesButton = requireElement<HTMLButtonElement>("#load-languages-button");
const apiRenderButton = requireElement<HTMLButtonElement>("#api-render-button");
const formatSelect = requireElement<HTMLSelectElement>("#format-select");
const graphFile = requireElement<HTMLInputElement>("#graph-file");
const graphInput = requireElement<HTMLTextAreaElement>("#graph-input");
const renderButton = requireElement<HTMLButtonElement>("#render-button");
const sampleButton = requireElement<HTMLButtonElement>("#sample-button");
const focusInput = requireElement<HTMLInputElement>("#focus-node");
const hopInput = requireElement<HTMLInputElement>("#focus-hops");
const diagram = requireElement<HTMLElement>("#diagram");
const stats = requireElement<HTMLElement>("#stats");
const serverStatus = requireElement<HTMLElement>("#server-status");
const mermaidOutput = requireElement<HTMLElement>("#mermaid-output");
const outputHeading = requireElement<HTMLElement>("#output-heading");
const errorOutput = requireElement<HTMLElement>("#error-output");
const violations = requireElement<HTMLElement>("#violations");
const downloadButton = requireElement<HTMLButtonElement>("#download-button");
const diagramTitle = requireElement<HTMLElement>("#diagram-title");

let currentOutput = "";
let currentOutputFormat: DiagramFormat = "svg";

graphInput.value = JSON.stringify(sampleGraph, null, 2);
render(sampleGraph);
void checkApiHealth();

scanButton.addEventListener("click", () => {
  void runAction(async () => {
    const graph = await scanRepository(repoPath.value.trim() || ".");
    graphInput.value = JSON.stringify(graph, null, 2);
    render(graph);
    setStatus(`Loaded ${graph.nodes.length} nodes from ${repoPath.value}`);
  });
});

checkButton.addEventListener("click", () => {
  void runAction(async () => {
    const result = await checkRepository(repoPath.value.trim() || ".");
    violations.innerHTML =
      result.length === 0
        ? `<p class="success">No rule violations detected.</p>`
        : result
            .map(
              (violation) =>
                `<article><strong>${escapeHtml(violation.rule_id)}</strong><p>${escapeHtml(
                  violation.message
                )}</p><code>${escapeHtml(violation.subject)}</code></article>`
            )
            .join("");
    setStatus(`Rules complete: ${result.length} violation${result.length === 1 ? "" : "s"}`);
  });
});

loadRulesButton.addEventListener("click", () => {
  void runAction(async () => {
    const rules = await listRules();
    outputHeading.textContent = "Available rules";
    mermaidOutput.textContent = rules
      .map(
        (rule) =>
          `${rule.id}\n  type: ${rule.type}\n  enabled: ${rule.enabled}\n  from: ${rule.from ?? "-"}\n  to: ${rule.to ?? "-"}`
      )
      .join("\n\n");
    setStatus(`Loaded ${rules.length} rules`);
  });
});

loadLanguagesButton.addEventListener("click", () => {
  void runAction(async () => {
    const languages = await listLanguages();
    outputHeading.textContent = "Supported languages";
    mermaidOutput.textContent = languages
      .map((language) => `${language.name} (${language.extensions.join(", ")})\n  extracts: ${language.extracts}`)
      .join("\n\n");
    setStatus(`Loaded ${languages.length} language recognizers`);
  });
});

apiRenderButton.addEventListener("click", () => {
  void runAction(async () => {
    const format = selectedFormat();
    const output = await renderDiagram(repoPath.value.trim() || ".", format);
    currentOutput = output;
    currentOutputFormat = format;
    diagramTitle.textContent = `API-rendered ${format.toUpperCase()} output`;
    if (format === "svg") {
      diagram.innerHTML = output;
    } else if (format === "html") {
      diagram.innerHTML = `<iframe title="Apex HTML diagram" srcdoc="${escapeAttribute(output)}"></iframe>`;
    } else {
      diagram.innerHTML = `<pre class="text-output">${escapeHtml(output)}</pre>`;
    }
    outputHeading.textContent = `Generated ${format.toUpperCase()}`;
    mermaidOutput.textContent = output;
    setStatus(`Rendered ${format.toUpperCase()} through local API`);
  });
});

renderButton.addEventListener("click", () => {
  try {
    const graph = parseGraphDocument(graphInput.value);
    const focusNode = focusInput.value.trim();
    const hops = Number.parseInt(hopInput.value, 10);
    render(focusNode.length > 0 ? focusGraph(graph, focusNode, Number.isFinite(hops) ? hops : 1) : graph);
    errorOutput.textContent = "";
  } catch (error) {
    errorOutput.textContent = error instanceof Error ? error.message : "Unknown graph rendering error";
  }
});

sampleButton.addEventListener("click", () => {
  graphInput.value = JSON.stringify(sampleGraph, null, 2);
  focusInput.value = "";
  render(sampleGraph);
});

graphFile.addEventListener("change", () => {
  const file = graphFile.files?.[0];
  if (file === undefined) {
    return;
  }
  void runAction(async () => {
    const text = await file.text();
    const graph = parseGraphDocument(text);
    graphInput.value = JSON.stringify(graph, null, 2);
    render(graph);
    setStatus(`Imported ${file.name}`);
  });
});

downloadButton.addEventListener("click", () => {
  if (currentOutput.length === 0) {
    errorOutput.textContent = "Render a diagram before downloading.";
    return;
  }
  const extension = currentOutputFormat === "mermaid" ? "mmd" : currentOutputFormat;
  const blob = new Blob([currentOutput], { type: mimeType(currentOutputFormat) });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = `apex-diagram.${extension}`;
  link.click();
  URL.revokeObjectURL(url);
});

function render(graph: GraphDocument): void {
  const currentStats = graphStats(graph);
  stats.textContent = `${currentStats.nodes} nodes · ${currentStats.edges} edges · layers: ${
    currentStats.layers.join(", ") || "none"
  }`;
  currentOutput = renderSvg(graph);
  currentOutputFormat = "svg";
  diagramTitle.textContent = "Interactive SVG preview";
  diagram.innerHTML = currentOutput;
  outputHeading.textContent = "Mermaid export";
  mermaidOutput.textContent = toMermaid(graph);
}

function requireElement<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (element === null) {
    throw new Error(`Apex UI failed to find required DOM node '${selector}'`);
  }
  return element;
}

async function checkApiHealth(): Promise<void> {
  try {
    const response = await fetch("/api/health");
    serverStatus.textContent = response.ok ? "Apex API connected" : "Apex API unavailable";
  } catch {
    serverStatus.textContent = "Run `apex ui` for repository-backed actions";
  }
}

async function runAction(action: () => Promise<void>): Promise<void> {
  setBusy(true);
  try {
    await action();
    errorOutput.textContent = "";
  } catch (error) {
    errorOutput.textContent = error instanceof Error ? error.message : "Unknown Apex UI error";
  } finally {
    setBusy(false);
  }
}

function selectedFormat(): DiagramFormat {
  const value = formatSelect.value;
  if (value === "svg" || value === "mermaid" || value === "html" || value === "json") {
    return value;
  }
  return "svg";
}

function setBusy(busy: boolean): void {
  for (const button of [
    scanButton,
    checkButton,
    loadRulesButton,
    loadLanguagesButton,
    apiRenderButton,
    renderButton,
    sampleButton,
    downloadButton
  ]) {
    button.disabled = busy;
  }
}

function setStatus(message: string): void {
  serverStatus.textContent = message;
}

function mimeType(format: DiagramFormat): string {
  switch (format) {
    case "svg":
      return "image/svg+xml";
    case "html":
      return "text/html";
    case "json":
      return "application/json";
    case "mermaid":
      return "text/plain";
  }
}

function escapeHtml(value: string): string {
  return value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function escapeAttribute(value: string): string {
  return escapeHtml(value).replaceAll('"', "&quot;");
}
