/**
 * Domain Services - Pure functions (parsing, extraction, transformation)
 * No ports - ports are in Application layer per Uncle Bob's Clean Architecture
 */

export { parseFrontmatter } from "./ParseFrontmatter.ts";
export { extractSections } from "./ExtractSections.ts";
export { extractIssueIdFromFilename } from "./ExtractIssueId.ts";
export { deriveTitleFromFolder } from "./DeriveTitle.ts";
export { parseIssueFile } from "./ParseIssueFile.ts";