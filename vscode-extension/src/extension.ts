import { renderSvg } from "../../ui/src/lib/renderer/svg_renderer.js";
import type { GraphDocument } from "../../ui/src/lib/types.js";

interface Disposable {
  dispose(): void;
}

interface ExtensionContext {
  readonly subscriptions: Disposable[];
}

export function activate(context: ExtensionContext): { renderClassDiagram: (graph: GraphDocument) => string } {
  const disposable: Disposable = { dispose: () => undefined };
  context.subscriptions.push(disposable);
  return { renderClassDiagram };
}

export function deactivate(): void {
  return undefined;
}

export function renderClassDiagram(graph: GraphDocument): string {
  return renderSvg(graph);
}

