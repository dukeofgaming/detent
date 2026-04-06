/**
 * Domain Services - Pure parsing functions
 */

import type { IssueFile, IssueFrontmatter } from "./types/issue.ts";

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

export function extractSections(content: string): Record<string, string> {
  const sections: Record<string, string> = {};
  const lines = content.split("\n");
  let currentSection: string | null = null;
  const currentContent: string[] = [];

  for (const line of lines) {
    const h2Match = line.match(/^##\s+(.+)/);
    if (h2Match) {
      if (currentSection) {
        sections[currentSection] = currentContent.join("\n").trim();
      }
      currentSection = h2Match[1].toLowerCase();
      currentContent.length = 0;
    } else if (currentSection) {
      currentContent.push(line);
    }
  }

  if (currentSection) {
    sections[currentSection] = currentContent.join("\n").trim();
  }

  return sections;
}

export function extractIssueIdFromFilename(filename: string): number | null {
  const match = filename.match(/^#?(\d+)/);
  return match ? parseInt(match[1], 10) : null;
}

export function deriveTitleFromFolder(folderName: string): string {
  const name = folderName.replace(/^\d+-/, "");
  const words = name.split(/[-_]/);
  return words
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

export function parseIssueFile(
  filepath: string,
  content: string,
  parentFolder?: string,
): IssueFile {
  const frontmatter = parseFrontmatter(content);
  const sections = extractSections(content);
  const filename = filepath.split("/").pop()?.replace(".md", "") || "";

  const issueIdFromFrontmatter = frontmatter?.issue?.id ?? null;
  const issueIdFromFilename = extractIssueIdFromFilename(filename);
  const issueId = issueIdFromFrontmatter || issueIdFromFilename;

  const title =
    frontmatter?.title ||
    content.match(/^#\s+(.+)/m)?.[1] ||
    (parentFolder ? deriveTitleFromFolder(parentFolder) : null) ||
    filename.replace(/^#/, "");

  return {
    filepath,
    filename,
    issueId,
    title,
    type: frontmatter?.type || "issue",
    author: frontmatter?.author,
    created: frontmatter?.created,
    updated: frontmatter?.updated,
    tags: frontmatter?.issue?.tags || [],
    sectionIds: frontmatter?.issue?.sections || {},
    sections,
  };
}
