/**
 * IssueAdapterPort - Port for GitHub issue operations (read-only)
 * (Copied from gh-push - vertical slices do not share code)
 * Note: This is a read-only version - gh-pull never modifies GitHub
 */

import type { GitHubIssue, GitHubComment } from "#domain/types";

export interface IssueMetadata {
  number: number;
  title: string;
  body: string;
  updatedAt: string;
}

export interface CommentMetadata {
  id: string;
  body: string;
  updatedAt: string;
  createdAt: string;
  author: string;
}

export interface IssueAdapterPort {
  findIssueByNumber(number: number): Promise<GitHubIssue | null>;
  findIssueByTitle(title: string): Promise<GitHubIssue | null>;
  getComments(issueNumber: number): Promise<GitHubComment[]>;
  
  // NEW: Metadata methods for pull operations
  getIssueMetadata(number: number): Promise<IssueMetadata | null>;
  getCommentsMetadata(issueNumber: number): Promise<CommentMetadata[]>;
}