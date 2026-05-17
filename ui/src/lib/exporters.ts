import type { GraphDocument } from "./types.js";

export function toMermaid(graph: GraphDocument): string {
  const classes = graph.nodes
    .filter((node) => node.kind === "type" || node.kind === "entity")
    .map((node) => `  class ${sanitize(node.name)}`)
    .join("\n");
  const edges = graph.edges
    .map((edge) => {
      const from = graph.nodes.find((node) => node.id === edge.from);
      const to = graph.nodes.find((node) => node.id === edge.to);
      if (from === undefined || to === undefined) {
        return "";
      }
      const arrow = edge.kind === "extends" ? "--|>" : edge.kind === "implements" ? "..|>" : "-->";
      return `  ${sanitize(from.name)} ${arrow} ${sanitize(to.name)}`;
    })
    .filter((line) => line.length > 0)
    .join("\n");
  return `classDiagram\n${classes}\n${edges}\n`;
}

function sanitize(value: string): string {
  return value.replace(/[^A-Za-z0-9_]/g, "_");
}

