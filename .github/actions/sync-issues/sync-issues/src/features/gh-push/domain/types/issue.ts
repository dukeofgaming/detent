/**
 * Domain Types - Core business entities
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
}

export interface GitHubIssue {
  number: number;
  title: string;
  body: string;
  labels: string[];
}

export interface GitHubComment {
  id: string;
  body: string;
}

export interface SyncResult {
  action: "created" | "updated" | "skipped" | "found";
  issueNumber: number | null;
  commentCount: number;
}

export interface SyncOptions {
  dryRun: boolean;
  verbose: boolean;
}
