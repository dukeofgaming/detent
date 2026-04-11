/**
 * ParseFrontmatter - Parse YAML frontmatter from markdown content
 * (Copied from gh-push - vertical slices do not share code)
 */

import type { IssueFrontmatter } from "#domain/types";

export function parseFrontmatter(content: string): IssueFrontmatter | null {
  const match = content.match(/^---\n([\s\S]*?)\n---\n/);
  if (!match) return null;

  const yamlLines = match[1].split("\n");
  const result: Record<string, unknown> = {};
  const issueData: Record<string, unknown> = {};
  let inIssueBlock = false;
  let inSectionsBlock = false;
  let sections: Record<string, string> = {};

  for (const line of yamlLines) {
    const trimmed = line.trim();
    const indent = line.length - line.trimStart().length;
    
    if (trimmed === "issue:") {
      inIssueBlock = true;
      inSectionsBlock = false;
      continue;
    }
    
    if (trimmed === "sections:") {
      inSectionsBlock = true;
      sections = {};
      issueData["sections"] = sections;
      continue;
    }
    
    if (inSectionsBlock) {
      const dashMatch = trimmed.match(/^- (\w+):\s*(\d+)/);
      if (dashMatch) {
        sections[dashMatch[1].toLowerCase()] = dashMatch[2];
        continue;
      }
      if (indent === 4 && trimmed.match(/^\w+:/)) {
        inSectionsBlock = false;
        const keyMatch = trimmed.match(/^(\w+):/);
        if (keyMatch) {
          const key = keyMatch[1];
          let value = trimmed.slice(trimmed.indexOf(":") + 1).trim();
          value = value.replace(/^["']|["']$/g, "");
          issueData[key] = value;
        }
        continue;
      }
    }
    
    if (inIssueBlock && !inSectionsBlock && indent === 4 && trimmed.match(/^\w+:/)) {
      const keyMatch = trimmed.match(/^(\w+):/);
      if (keyMatch) {
        const key = keyMatch[1];
        let value = trimmed.slice(trimmed.indexOf(":") + 1).trim();
        value = value.replace(/^["']|["']$/g, "");
        issueData[key] = value;
      }
      continue;
    }
    
    if (!inIssueBlock && !inSectionsBlock && trimmed.includes(":")) {
      const colonIdx = line.indexOf(":");
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
  }

  if (Object.keys(issueData).length > 0) {
    result["issue"] = issueData;
  }

  return result as unknown as IssueFrontmatter;
}