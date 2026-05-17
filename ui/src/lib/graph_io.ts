import type { GraphDocument, GraphEdge, GraphNode } from "./types.js";

const nodeKinds = new Set(["type", "entity", "file"]);
const edgeKinds = new Set(["imports", "extends", "implements", "relates_to", "contains"]);

export function parseGraphDocument(input: string): GraphDocument {
  const parsed: unknown = JSON.parse(input);
  if (!isRecord(parsed) || !Array.isArray(parsed.nodes) || !Array.isArray(parsed.edges)) {
    throw new Error("Graph JSON must contain nodes and edges arrays");
  }
  return {
    nodes: parsed.nodes.map(parseNode),
    edges: parsed.edges.map(parseEdge)
  };
}

export function graphStats(graph: GraphDocument): { nodes: number; edges: number; layers: readonly string[] } {
  const layers = new Set(
    graph.nodes
      .map((node) => node.layer)
      .filter((layer): layer is string => layer !== null && layer.length > 0)
  );
  return {
    nodes: graph.nodes.length,
    edges: graph.edges.length,
    layers: [...layers].sort()
  };
}

function parseNode(value: unknown): GraphNode {
  if (!isRecord(value)) {
    throw new Error("Graph node must be an object");
  }
  const { id, name, kind, path, layer } = value;
  if (typeof id !== "string" || typeof name !== "string" || typeof kind !== "string" || typeof path !== "string") {
    throw new Error("Graph node is missing required string fields");
  }
  if (!nodeKinds.has(kind)) {
    throw new Error(`Unsupported graph node kind '${kind}'`);
  }
  if (layer !== null && typeof layer !== "string") {
    throw new Error("Graph node layer must be a string or null");
  }
  return { id, name, kind: kind as GraphNode["kind"], path, layer };
}

function parseEdge(value: unknown): GraphEdge {
  if (!isRecord(value)) {
    throw new Error("Graph edge must be an object");
  }
  const { from, to, kind } = value;
  if (typeof from !== "string" || typeof to !== "string" || typeof kind !== "string") {
    throw new Error("Graph edge is missing required string fields");
  }
  if (!edgeKinds.has(kind)) {
    throw new Error(`Unsupported graph edge kind '${kind}'`);
  }
  return { from, to, kind: kind as GraphEdge["kind"] };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

