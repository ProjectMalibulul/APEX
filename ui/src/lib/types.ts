export type GraphNodeKind = "type" | "entity" | "file";
export type GraphEdgeKind = "imports" | "extends" | "implements" | "relates_to" | "contains";

export interface GraphNode {
  readonly id: string;
  readonly name: string;
  readonly kind: GraphNodeKind;
  readonly path: string;
  readonly layer: string | null;
}

export interface GraphEdge {
  readonly from: string;
  readonly to: string;
  readonly kind: GraphEdgeKind;
}

export interface GraphDocument {
  readonly nodes: readonly GraphNode[];
  readonly edges: readonly GraphEdge[];
}

export interface Point {
  readonly x: number;
  readonly y: number;
}

