/**
 * Domain Services - Pure parsing functions
 * No side effects - no I/O
 */

/**
 * Parse YAML frontmatter from markdown content
 * @param {string} content
 * @returns {import('../types/issue.mjs').IssueFrontmatter | null}
 */
export function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---\n/);
  if (!match) return null;

  const yaml = match[1];
  /** @type {Record<string, unknown>} */
  const result = {};

  for (const line of yaml.split('\n')) {
    const colonIdx = line.indexOf(':');
    if (colonIdx === -1) continue;

    const key = line.slice(0, colonIdx).trim();
    let value = line.slice(colonIdx + 1).trim();

    if (!value) continue;

    if (value.startsWith('[')) {
      result[key] = value.replace(/[\[\]]/g, '').split(',').map(v => v.trim());
    } else if (value.startsWith('{') || value.startsWith('-')) {
      continue;
    } else {
      result[key] = value.replace(/^["']|["']$/g, '');
    }
  }

  return result;
}

/**
 * Extract all ## sections from markdown content
 * @param {string} content
 * @returns {Record<string, string>}
 */
export function extractSections(content) {
  const sections = {};
  const lines = content.split('\n');
  let currentSection = null;
  let currentContent = [];

  for (const line of lines) {
    const h2Match = line.match(/^##\s+(.+)/);
    if (h2Match) {
      if (currentSection) {
        sections[currentSection] = currentContent.join('\n').trim();
      }
      currentSection = h2Match[1].toLowerCase();
      currentContent = [];
    } else if (currentSection) {
      currentContent.push(line);
    }
  }

  if (currentSection) {
    sections[currentSection] = currentContent.join('\n').trim();
  }

  return sections;
}

/**
 * Extract GitHub issue number from filename
 * Supports: #3.md, 3.md, 3-convert-bpmn-to-mdx.md
 * @param {string} filename
 * @returns {number | null}
 */
export function extractIssueIdFromFilename(filename) {
  const match = filename.match(/^#?(\d+)/);
  return match ? parseInt(match[1], 10) : null;
}

/**
 * Derive title from parent folder name
 * e.g., "3-convert-bpmn-to-mdx" -> "Convert BPMN to MDX"
 * @param {string} folderName
 * @returns {string}
 */
export function deriveTitleFromFolder(folderName) {
  const name = folderName.replace(/^\d+-/, '');
  const words = name.split(/[-_]/);
  return words
    .map(w => w.charAt(0).toUpperCase() + w.slice(1))
    .join(' ');
}

/**
 * Parse an issue file into an IssueFile domain object
 * @param {string} filepath
 * @param {string} content
 * @param {string} [parentFolder]
 * @returns {import('../types/issue.mjs').IssueFile}
 */
export function parseIssueFile(filepath, content, parentFolder) {
  const frontmatter = parseFrontmatter(content);
  const sections = extractSections(content);
  const filename = filepath.split('/').pop()?.replace('.md', '') || '';

  const issueIdFromFrontmatter = frontmatter?.issue?.id ?? null;
  const issueIdFromFilename = extractIssueIdFromFilename(filename);
  const issueId = issueIdFromFrontmatter || issueIdFromFilename;

  const title = frontmatter?.title ||
    content.match(/^#\s+(.+)/m)?.[1] ||
    (parentFolder ? deriveTitleFromFolder(parentFolder) : null) ||
    filename.replace(/^#/, '');

  return {
    filepath,
    filename,
    issueId,
    title,
    type: frontmatter?.type || 'issue',
    author: frontmatter?.author,
    created: frontmatter?.created,
    updated: frontmatter?.updated,
    tags: frontmatter?.issue?.tags || [],
    sectionIds: frontmatter?.issue?.sections || {},
    sections,
  };
}
