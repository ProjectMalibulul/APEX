import { layeredLayout } from "../layout.js";
import type { GraphDocument, GraphEdge, GraphMember, GraphNode } from "../types.js";

const NODE_W = 200;
const NODE_HEADER = 40;
const MEMBER_LINE = 16;
const MEMBER_PAD = 10;
const COL_W = 240;
const ROW_H = 160;
const PAD = 30;
const MAX_MEMBERS = 5;

export function renderSvg(graph: GraphDocument): string {
  const diagram = diagramGraph(graph);
  if (diagram.nodes.length === 0) {
    return emptySvg();
  }
  const layout = layeredLayout(diagram);
  const nodesById = new Map<string, GraphNode>(diagram.nodes.map((n) => [n.id, n]));
  let maxX = 0;
  let maxY = 0;
  const nodes = diagram.nodes
    .map((node) => {
      const point = layout.get(node.id);
      if (point === undefined) return "";
      const x = PAD + point.x * (COL_W / 180);
      const y = PAD + point.y * (ROW_H / 110);
      const h = nodeHeight(node);
      maxX = Math.max(maxX, x + NODE_W);
      maxY = Math.max(maxY, y + h);
      return renderNode(node, x, y, h);
    })
    .join("");
  const edges = diagram.edges
    .map((edge) => {
      const from = layout.get(edge.from);
      const to = layout.get(edge.to);
      if (from === undefined || to === undefined) return "";
      const fromNode = nodesById.get(edge.from);
      const toNode = nodesById.get(edge.to);
      if (!fromNode || !toNode) return "";
      const fx = PAD + from.x * (COL_W / 180) + NODE_W / 2;
      const fy = PAD + from.y * (ROW_H / 110) + nodeHeight(fromNode) / 2;
      const tx = PAD + to.x * (COL_W / 180) + NODE_W / 2;
      const ty = PAD + to.y * (ROW_H / 110) + nodeHeight(toNode) / 2;
      return renderEdge(edge, fx, fy, tx, ty);
    })
    .join("");
  const width = Math.max(maxX + PAD, 320);
  const height = Math.max(maxY + PAD, 200);
  return `<svg xmlns="http://www.w3.org/2000/svg" role="img" aria-labelledby="apex-title" viewBox="0 0 ${width} ${height}" width="100%" height="100%" preserveAspectRatio="xMidYMid meet"><title id="apex-title">Apex architecture diagram</title>${defs()}${edges}${nodes}</svg>`;
}

function defs(): string {
  return [
    arrowMarker("apex-arrow-imports", "#1f6feb"),
    arrowMarker("apex-arrow-extends", "#8957e5"),
    arrowMarker("apex-arrow-implements", "#2da44e"),
    arrowMarker("apex-arrow-relates", "#9da7b3")
  ].join("");
}

function arrowMarker(id: string, color: string): string {
  return `<defs><marker id="${id}" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="${color}"/></marker></defs>`;
}

function renderNode(node: GraphNode, x: number, y: number, h: number): string {
  const name = escapeXml(node.name);
  const sub = escapeXml(node.layer ?? shortPath(node.path));
  const members = (node.members ?? []).slice(0, MAX_MEMBERS);
  const extra = (node.members?.length ?? 0) - members.length;
  const memberLines = members
    .map((m, i) => {
      const yLine = y + NODE_HEADER + 16 + i * MEMBER_LINE;
      return `<text x="${x + MEMBER_PAD}" y="${yLine}" font-family="ui-monospace, Menlo, monospace" font-size="11" fill="#1f2937">${escapeXml(memberLabel(m))}</text>`;
    })
    .join("");
  const more =
    extra > 0
      ? `<text x="${x + MEMBER_PAD}" y="${y + NODE_HEADER + 16 + members.length * MEMBER_LINE}" font-family="ui-monospace, Menlo, monospace" font-size="11" fill="#6b7280">…(+${extra} more)</text>`
      : "";
  return `<g tabindex="0" aria-label="${name}"><rect x="${x}" y="${y}" rx="10" ry="10" width="${NODE_W}" height="${h}" fill="#ffffff" stroke="#1f2937" stroke-width="1.2"/><rect x="${x}" y="${y}" rx="10" ry="10" width="${NODE_W}" height="${NODE_HEADER}" fill="#1f6feb"/><text x="${x + NODE_W / 2}" y="${y + 18}" text-anchor="middle" font-family="ui-sans-serif, system-ui" font-weight="700" font-size="13" fill="#ffffff">${name}</text><text x="${x + NODE_W / 2}" y="${y + 32}" text-anchor="middle" font-family="ui-sans-serif, system-ui" font-size="10" fill="#dbe7ff">${sub}</text><line x1="${x}" y1="${y + NODE_HEADER}" x2="${x + NODE_W}" y2="${y + NODE_HEADER}" stroke="#d1d5db"/>${memberLines}${more}</g>`;
}

function renderEdge(edge: GraphEdge, fx: number, fy: number, tx: number, ty: number): string {
  const style = edgeStyle(edge.kind);
  const dash = style.dashed ? ' stroke-dasharray="5 4"' : "";
  const label = escapeXml(style.label);
  const midX = (fx + tx) / 2;
  const midY = (fy + ty) / 2;
  return `<g class="apex-edge"><line x1="${fx}" y1="${fy}" x2="${tx}" y2="${ty}" stroke="${style.color}" stroke-width="1.4"${dash} marker-end="url(#${style.marker})"/><text x="${midX}" y="${midY - 4}" text-anchor="middle" font-family="ui-sans-serif, system-ui" font-size="9" fill="${style.color}">${label}</text></g>`;
}

function edgeStyle(kind: GraphEdge["kind"]): {
  color: string;
  marker: string;
  dashed: boolean;
  label: string;
} {
  switch (kind) {
    case "extends":
      return { color: "#8957e5", marker: "apex-arrow-extends", dashed: false, label: "extends" };
    case "implements":
      return { color: "#2da44e", marker: "apex-arrow-implements", dashed: true, label: "implements" };
    case "imports":
      return { color: "#1f6feb", marker: "apex-arrow-imports", dashed: false, label: "imports" };
    case "relates_to":
      return { color: "#9da7b3", marker: "apex-arrow-relates", dashed: true, label: "relates" };
    default:
      return { color: "#9da7b3", marker: "apex-arrow-relates", dashed: false, label: "" };
  }
}

function nodeHeight(node: GraphNode): number {
  const memberCount = Math.min(node.members?.length ?? 0, MAX_MEMBERS);
  const more = (node.members?.length ?? 0) > MAX_MEMBERS ? 1 : 0;
  return NODE_HEADER + 12 + (memberCount + more) * MEMBER_LINE + 8;
}

function memberLabel(member: GraphMember): string {
  const sigil = member.kind === "method" ? "()" : member.kind === "constructor" ? "{}" : "";
  const vis = member.visibility ? `${member.visibility} ` : "";
  return `${vis}${member.name}${sigil}`;
}

function shortPath(path: string): string {
  if (path.length <= 26) return path;
  const parts = path.split(/[\\/]/);
  if (parts.length <= 2) return path.slice(-26);
  return `…/${parts.slice(-2).join("/")}`;
}

function diagramGraph(graph: GraphDocument): GraphDocument {
  const nodeIds = new Set(
    graph.nodes.filter((node) => node.kind === "type" || node.kind === "entity").map((node) => node.id)
  );
  return {
    nodes: graph.nodes.filter((node) => nodeIds.has(node.id)),
    edges: graph.edges.filter((edge) => edge.kind !== "contains" && nodeIds.has(edge.from) && nodeIds.has(edge.to))
  };
}

function emptySvg(): string {
  return `<svg xmlns="http://www.w3.org/2000/svg" role="img" aria-labelledby="apex-title" viewBox="0 0 320 120" width="100%" height="100%" preserveAspectRatio="xMidYMid meet"><title id="apex-title">Apex architecture diagram</title><text x="160" y="60" text-anchor="middle" font-family="ui-sans-serif, system-ui" font-size="14" fill="#6b7280">No diagrammable nodes yet</text></svg>`;
}

function escapeXml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}
