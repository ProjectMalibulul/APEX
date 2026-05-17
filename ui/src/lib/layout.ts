import type { GraphDocument, Point } from "./types.js";

export function layeredLayout(graph: GraphDocument): ReadonlyMap<string, Point> {
  const sorted = [...graph.nodes].sort((left, right) => left.id.localeCompare(right.id));
  return new Map(
    sorted.map((node, index) => [
      node.id,
      {
        x: 40 + (index % 4) * 180,
        y: 40 + Math.floor(index / 4) * 110
      }
    ])
  );
}

export function focusGraph(graph: GraphDocument, nodeId: string, hops: number): GraphDocument {
  const seen = new Set<string>([nodeId]);
  const queue: Array<{ id: string; depth: number }> = [{ id: nodeId, depth: 0 }];
  while (queue.length > 0) {
    const current = queue.shift();
    if (current === undefined || current.depth >= hops) {
      continue;
    }
    for (const edge of graph.edges) {
      const next = edge.from === current.id ? edge.to : edge.to === current.id ? edge.from : null;
      if (next !== null && !seen.has(next)) {
        seen.add(next);
        queue.push({ id: next, depth: current.depth + 1 });
      }
    }
  }
  return {
    nodes: graph.nodes.filter((node) => seen.has(node.id)),
    edges: graph.edges.filter((edge) => seen.has(edge.from) && seen.has(edge.to))
  };
}

