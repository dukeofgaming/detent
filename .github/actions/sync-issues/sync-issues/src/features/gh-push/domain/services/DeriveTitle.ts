/**
 * DeriveTitleFromFolder - Derive title from folder name
 */

export function deriveTitleFromFolder(folderName: string): string {
  const name = folderName.replace(/^\d+-/, "");
  const words = name.split(/[-_]/);
  return words
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}