/**
 * ParseFrontmatter - Parse YAML frontmatter from markdown content
 */

import type { IssueFrontmatter } from "#domain/types";

export function parseFrontmatter(content: string): IssueFrontmatter | null {
  const match = content.match(/^---\n([\s\S]*?)\n---\n/);
  if (!match) return null;

  const yaml = match[1];
  const result: Record<string, unknown> = {};

  for (const line of yaml.split("\n")) {
    const colonIdx = line.indexOf(":");
    if (colonIdx === -1) continue;

    const key = line.slice(0, colonIdx).trim();
    let value = line.slice(colonIdx + 1).trim();

    if (!value) continue;

    if (value.startsWith("[")) {
      result[key] = value.replace(/[\[\]]/g, "").split(",").map((v) => v.trim());
    } else if (value.startsWith("{") || value.startsWith("-")) {
      continue;
    } else {
      result[key] = value.replace(/^["']|["']$/g, "");
    }
  }

  return result as unknown as IssueFrontmatter;
}