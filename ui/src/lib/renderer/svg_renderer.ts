import { layeredLayout } from "../layout.js";
import type { GraphDocument } from "../types.js";

export function renderSvg(graph: GraphDocument): string {
  const layout = layeredLayout(graph);
  const width = Math.max(260, layout.size * 180 + 40);
  const maxY = Math.max(140, ...[...layout.values()].map((point) => point.y + 100));
  const lines = graph.edges
    .map((edge) => {
      const from = layout.get(edge.from);
      const to = layout.get(edge.to);
      if (from === undefined || to === undefined) {
        return "";
      }
      return `<line x1="${from.x + 60}" y1="${from.y + 24}" x2="${to.x + 60}" y2="${to.y + 24}" stroke="#637083" stroke-width="2" />`;
    })
    .join("");
  const nodes = graph.nodes
    .map((node) => {
      const point = layout.get(node.id);
      if (point === undefined) {
        return "";
      }
      const name = escapeXml(node.name);
      return `<g tabindex="0" aria-label="${name}"><rect x="${point.x}" y="${point.y}" width="120" height="48" rx="8" fill="#f7fbff" stroke="#1f6feb"/><text x="${point.x + 60}" y="${point.y + 29}" text-anchor="middle" font-family="sans-serif" font-size="13">${name}</text></g>`;
    })
    .join("");
  return `<svg xmlns="http://www.w3.org/2000/svg" role="img" aria-labelledby="title" viewBox="0 0 ${width} ${maxY}"><title id="title">Apex architecture diagram</title>${lines}${nodes}</svg>`;
}

function escapeXml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

