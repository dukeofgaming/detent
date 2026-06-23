/**
 * IssueAdapterPort - Interface for GitHub issue operations
 * Port interface in Application layer per Uncle Bob's Clean Architecture
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
  createIssue(title: string, body: string, labels: string[]): Promise<GitHubIssue>;
  updateIssue(issueNumber: number, title: string, body: string): Promise<void>;
  getComments(issueNumber: number): Promise<GitHubComment[]>;
  createComment(issueNumber: number, body: string): Promise<GitHubComment>;
  updateComment(commentId: string, body: string): Promise<void>;
  
  // NEW: Metadata methods for upstream change detection
  getIssueMetadata(number: number): Promise<IssueMetadata | null>;
  getCommentsMetadata(issueNumber: number): Promise<CommentMetadata[]>;
}