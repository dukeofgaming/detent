/**
 * ParseIssueFile - Parse an issue file into an IssueFile domain object
 */

import type { IssueFile } from "#domain/types";
import { parseFrontmatter } from "./ParseFrontmatter.ts";
import { extractSections } from "./ExtractSections.ts";
import { extractIssueIdFromFilename } from "./ExtractIssueId.ts";
import { deriveTitleFromFolder } from "./DeriveTitle.ts";

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