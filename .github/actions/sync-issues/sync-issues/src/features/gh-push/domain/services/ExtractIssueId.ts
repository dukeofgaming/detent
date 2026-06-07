/**
 * ExtractIssueIdFromFilename - Extract issue ID from filename
 */

export function extractIssueIdFromFilename(filename: string): number | null {
  const match = filename.match(/^#?(\d+)/);
  return match ? parseInt(match[1], 10) : null;
}