import { parseGraphDocument } from "./graph_io.js";
import type { GraphDocument } from "./types.js";

export interface RuleViolation {
  readonly rule_id: string;
  readonly message: string;
  readonly subject: string;
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

