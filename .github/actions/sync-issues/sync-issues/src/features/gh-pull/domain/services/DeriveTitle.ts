/**
 * DeriveTitleFromFolder - Derive title from folder name
 * (Copied from gh-push - vertical slices do not share code)
 */

export function deriveTitleFromFolder(folderName: string): string {
  return folderName
    .split(/[-_]/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}