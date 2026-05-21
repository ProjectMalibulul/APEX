import {
  checkRepository,
  getMetrics,
  listLanguages,
  listRules,
  renderDiagram,
  scanRepository,
  type DiagramFormat,
  type GraphMetrics
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
const diagramViewport = requireElement<HTMLElement>("#diagram-viewport");
const stats = requireElement<HTMLElement>("#stats");
const serverStatus = requireElement<HTMLElement>("#server-status");
const mermaidOutput = requireElement<HTMLElement>("#mermaid-output");
const outputHeading = requireElement<HTMLElement>("#output-heading");
const errorOutput = requireElement<HTMLElement>("#error-output");
const violations = requireElement<HTMLElement>("#violations");
const downloadButton = requireElement<HTMLButtonElement>("#download-button");
const diagramTitle = requireElement<HTMLElement>("#diagram-title");
const zoomIn = requireElement<HTMLButtonElement>("#zoom-in");
const zoomOut = requireElement<HTMLButtonElement>("#zoom-out");
const zoomFit = requireElement<HTMLButtonElement>("#zoom-fit");
const zoomReset = requireElement<HTMLButtonElement>("#zoom-reset");
const metricsButton = requireElement<HTMLButtonElement>("#metrics-button");
const metricsPanel = requireElement<HTMLElement>("#metrics-panel");

let currentOutput = "";
let currentOutputFormat: DiagramFormat = "svg";

const view = { x: 0, y: 0, scale: 1 };

function applyView(): void {
  diagram.style.transform = `translate(${view.x}px, ${view.y}px) scale(${view.scale})`;
}

function setScale(target: number, anchorX = diagramViewport.clientWidth / 2, anchorY = diagramViewport.clientHeight / 2): void {
  const next = Math.max(0.1, Math.min(8, target));
  const ratio = next / view.scale;
  view.x = anchorX - (anchorX - view.x) * ratio;
  view.y = anchorY - (anchorY - view.y) * ratio;
  view.scale = next;
  applyView();
}

function fitToView(): void {
  const svg = diagram.querySelector("svg");
  if (!svg) return;
  const vb = svg.getAttribute("viewBox");
  if (!vb) return;
  const parts = vb.split(/\s+/).map(Number);
  if (parts.length !== 4) return;
  const [, , w, h] = parts;
  if (!Number.isFinite(w) || !Number.isFinite(h) || w <= 0 || h <= 0) return;
  const vpW = diagramViewport.clientWidth;
  const vpH = diagramViewport.clientHeight;
  if (vpW === 0 || vpH === 0) return;
  const scale = Math.min(vpW / w, vpH / h) * 0.95;
  view.scale = scale;
  view.x = (vpW - w * scale) / 2;
  view.y = (vpH - h * scale) / 2;
  applyView();
}

function resetView(): void {
  view.x = 0;
  view.y = 0;
  view.scale = 1;
  applyView();
}

diagramViewport.addEventListener(
  "wheel",
  (event) => {
    event.preventDefault();
    const rect = diagramViewport.getBoundingClientRect();
    const ax = event.clientX - rect.left;
    const ay = event.clientY - rect.top;
    const factor = event.deltaY > 0 ? 0.9 : 1.1;
    setScale(view.scale * factor, ax, ay);
  },
  { passive: false }
);

let panState: { startX: number; startY: number; baseX: number; baseY: number; pointerId: number } | null = null;
diagramViewport.addEventListener("pointerdown", (event) => {
  if (event.button !== 0) return;
  panState = {
    startX: event.clientX,
    startY: event.clientY,
    baseX: view.x,
    baseY: view.y,
    pointerId: event.pointerId
  };
  diagramViewport.classList.add("is-panning");
  diagramViewport.setPointerCapture(event.pointerId);
});
diagramViewport.addEventListener("pointermove", (event) => {
  if (!panState || event.pointerId !== panState.pointerId) return;
  view.x = panState.baseX + (event.clientX - panState.startX);
  view.y = panState.baseY + (event.clientY - panState.startY);
  applyView();
});
const endPan = (event: PointerEvent): void => {
  if (!panState || event.pointerId !== panState.pointerId) return;
  panState = null;
  diagramViewport.classList.remove("is-panning");
};
diagramViewport.addEventListener("pointerup", endPan);
diagramViewport.addEventListener("pointercancel", endPan);
diagramViewport.addEventListener("pointerleave", endPan);

zoomIn.addEventListener("click", () => setScale(view.scale * 1.2));
zoomOut.addEventListener("click", () => setScale(view.scale / 1.2));
zoomFit.addEventListener("click", () => fitToView());
zoomReset.addEventListener("click", () => resetView());

metricsButton.addEventListener("click", () => {
  void runAction(async () => {
    const m = await getMetrics(repoPath.value.trim() || ".");
    renderMetrics(m);
    setStatus(`Metrics: ${m.node_count} nodes, ${m.cycles.length} cycles, ${m.component_count} components`);
  });
});

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
      resetView();
      requestAnimationFrame(() => fitToView());
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
  resetView();
  requestAnimationFrame(() => fitToView());
}

function renderMetrics(m: GraphMetrics): void {
  metricsPanel.hidden = false;
  const card = (label: string, value: string | number): string =>
    `<div class="metric-card"><span>${escapeHtml(label)}</span><strong>${escapeHtml(String(value))}</strong></div>`;
  const grid =
    `<div class="metric-grid">` +
    card("nodes", m.node_count) +
    card("edges", m.edge_count) +
    card("components", m.component_count) +
    card("cycles", m.cycles.length) +
    card("orphans", m.orphans.length) +
    `</div>`;
  const hotspots =
    m.hotspots.length === 0
      ? ""
      : `<h3>Hotspots</h3><ul>${m.hotspots
          .slice(0, 8)
          .map((h) => `<li><strong>${escapeHtml(h.name)}</strong> · in=${h.fan_in} · out=${h.fan_out}</li>`)
          .join("")}</ul>`;
  const cycles =
    m.cycles.length === 0
      ? ""
      : `<h3>Import cycles</h3><ul>${m.cycles
          .map((c) => `<li>${escapeHtml(c.join(" → "))}</li>`)
          .join("")}</ul>`;
  const layers =
    Object.keys(m.layer_mix).length === 0
      ? ""
      : `<h3>Layers</h3><ul>${Object.entries(m.layer_mix)
          .map(([layer, count]) => `<li>${escapeHtml(layer)}: ${count}</li>`)
          .join("")}</ul>`;
  metricsPanel.innerHTML = grid + hotspots + cycles + layers;
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
    downloadButton,
    metricsButton
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
