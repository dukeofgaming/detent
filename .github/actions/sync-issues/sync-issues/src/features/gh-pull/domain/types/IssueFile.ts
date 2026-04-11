/**
 * IssueFile - Represents a parsed issue file from docs/issues/
 */

export interface IssueFrontmatter {
  type: "issue" | "journal";
  title?: string;
  author?: string;
  created?: string;
  updated?: string;
  issue?: {
    id?: number;
    tags?: string[];
    sections?: Record<string, string>;
    lastSyncedAt?: string;
    canonicalSections?: string[];
  };
}

export interface IssueFile {
  filepath: string;
  filename: string;
  issueId: number | null;
  title: string;
  type: "issue" | "journal";
  author?: string;
  created?: string;
  updated?: string;
  tags: string[];
  sectionIds: Record<string, string>;
  sections: Record<string, string>;
  frontmatter: IssueFrontmatter | null;
}