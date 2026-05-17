import { renderSvg } from "../../ui/src/lib/renderer/svg_renderer.js";
import { parseGraphDocument } from "../../ui/src/lib/graph_io.js";
import type { GraphDocument } from "../../ui/src/lib/types.js";

interface Disposable {
  dispose(): void;
}

interface ExtensionContext {
  readonly subscriptions: Disposable[];
}

export function activate(context: ExtensionContext): {
  renderClassDiagram: (graph: GraphDocument) => string;
  renderWebviewHtml: (graph: GraphDocument) => string;
  parseCliGraph: (json: string) => GraphDocument;
} {
  const disposable: Disposable = { dispose: () => undefined };
  context.subscriptions.push(disposable);
  return { renderClassDiagram, renderWebviewHtml, parseCliGraph };
}

export function deactivate(): void {
  return undefined;
}

export function renderClassDiagram(graph: GraphDocument): string {
  return renderSvg(graph);
}

export function renderWebviewHtml(graph: GraphDocument): string {
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><style>${webviewCss()}</style></head><body><main><h1>Apex Class Diagram</h1><p>${graph.nodes.length} nodes · ${graph.edges.length} edges</p><section>${renderSvg(graph)}</section></main></body></html>`;
}

export function parseCliGraph(json: string): GraphDocument {
  return parseGraphDocument(json);
}

function webviewCss(): string {
  return "body{margin:0;padding:20px;background:#0d1117;color:#e6edf3;font-family:system-ui,sans-serif}section{overflow:auto;background:#fff;border-radius:12px;padding:20px}h1{margin-top:0}";
}
