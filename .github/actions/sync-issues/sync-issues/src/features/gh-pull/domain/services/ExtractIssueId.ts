/**
 * ExtractIssueIdFromFilename - Extract numeric issue ID from filename
 * (Copied from gh-push - vertical slices do not share code)
 */

export function extractIssueIdFromFilename(filename: string): number | null {
  const match = filename.match(/^#?(\d+)/);
  return match ? parseInt(match[1], 10) : null;
}