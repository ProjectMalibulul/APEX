import { parseGraphDocument } from "./graph_io.js";
import type { GraphDocument } from "./types.js";

export interface RuleViolation {
  readonly rule_id: string;
  readonly message: string;
  readonly subject: string;
}

export interface RuleDefinition {
  readonly id: string;
  readonly type: string;
  readonly from: string | null;
  readonly to: string | null;
  readonly enabled: boolean;
}

export interface LanguageSupport {
  readonly name: string;
  readonly extensions: readonly string[];
  readonly extracts: string;
}

export interface GraphMetrics {
  readonly node_count: number;
  readonly edge_count: number;
  readonly component_count: number;
  readonly hotspots: readonly { id: string; name: string; fan_in: number; fan_out: number }[];
  readonly cycles: readonly (readonly string[])[];
  readonly orphans: readonly string[];
  readonly layer_mix: Record<string, number>;
  readonly layer_edges: readonly { from: string; to: string; count: number }[];
}

export async function getMetrics(path: string): Promise<GraphMetrics> {
  const response = await fetch(buildApiUrl("metrics", { path }));
  const text = await requireOk(response);
  const parsed: unknown = JSON.parse(text);
  if (typeof parsed !== "object" || parsed === null) {
    throw new Error("Metrics API returned a non-object response");
  }
  return parsed as GraphMetrics;
}

export type DiagramFormat = "svg" | "mermaid" | "html" | "json";

export function buildApiUrl(route: string, params: Record<string, string>): string {
  const query = new URLSearchParams(params);
  return `/api/${route}?${query.toString()}`;
}

export async function scanRepository(path: string): Promise<GraphDocument> {
  const response = await fetch(buildApiUrl("scan", { path }));
  const text = await requireOk(response);
  return parseGraphDocument(text);
}

export async function checkRepository(path: string): Promise<readonly RuleViolation[]> {
  const response = await fetch(buildApiUrl("check", { path }));
  const text = await requireOk(response);
  const parsed: unknown = JSON.parse(text);
  if (!Array.isArray(parsed)) {
    throw new Error("Rule check API returned a non-array response");
  }
  return parsed.map(parseViolation);
}

export async function renderDiagram(path: string, format: DiagramFormat): Promise<string> {
  const response = await fetch(buildApiUrl("diagram", { path, format }));
  return requireOk(response);
}

export async function listRules(): Promise<readonly RuleDefinition[]> {
  const response = await fetch("/api/rules");
  const text = await requireOk(response);
  const parsed: unknown = JSON.parse(text);
  if (!Array.isArray(parsed)) {
    throw new Error("Rules API returned a non-array response");
  }
  return parsed.map(parseRuleDefinition);
}

export async function listLanguages(): Promise<readonly LanguageSupport[]> {
  const response = await fetch("/api/languages");
  const text = await requireOk(response);
  const parsed: unknown = JSON.parse(text);
  if (!Array.isArray(parsed)) {
    throw new Error("Languages API returned a non-array response");
  }
  return parsed.map(parseLanguageSupport);
}

async function requireOk(response: Response): Promise<string> {
  const text = await response.text();
  if (!response.ok) {
    throw new Error(extractError(text) ?? `Apex API returned HTTP ${response.status}`);
  }
  return text;
}

function parseViolation(value: unknown): RuleViolation {
  if (typeof value !== "object" || value === null) {
    throw new Error("Rule violation must be an object");
  }
  const record = value as Record<string, unknown>;
  if (
    typeof record.rule_id !== "string" ||
    typeof record.message !== "string" ||
    typeof record.subject !== "string"
  ) {
    throw new Error("Rule violation is missing required fields");
  }
  return {
    rule_id: record.rule_id,
    message: record.message,
    subject: record.subject
  };
}

function parseRuleDefinition(value: unknown): RuleDefinition {
  if (typeof value !== "object" || value === null) {
    throw new Error("Rule definition must be an object");
  }
  const record = value as Record<string, unknown>;
  if (
    typeof record.id !== "string" ||
    typeof record.type !== "string" ||
    typeof record.enabled !== "boolean" ||
    (record.from !== null && typeof record.from !== "string") ||
    (record.to !== null && typeof record.to !== "string")
  ) {
    throw new Error("Rule definition is missing required fields");
  }
  return {
    id: record.id,
    type: record.type,
    from: record.from,
    to: record.to,
    enabled: record.enabled
  };
}

function parseLanguageSupport(value: unknown): LanguageSupport {
  if (typeof value !== "object" || value === null) {
    throw new Error("Language support entry must be an object");
  }
  const record = value as Record<string, unknown>;
  if (
    typeof record.name !== "string" ||
    !Array.isArray(record.extensions) ||
    !record.extensions.every((extension) => typeof extension === "string") ||
    typeof record.extracts !== "string"
  ) {
    throw new Error("Language support entry is missing required fields");
  }
  return {
    name: record.name,
    extensions: record.extensions,
    extracts: record.extracts
  };
}

function extractError(text: string): string | null {
  try {
    const parsed: unknown = JSON.parse(text);
    if (typeof parsed === "object" && parsed !== null && typeof (parsed as Record<string, unknown>).error === "string") {
      return (parsed as { error: string }).error;
    }
  } catch {
    return null;
  }
  return null;
}
