/**
 * GitHub Issue Adapter Port - Interface for GitHub operations
 */

import type { GitHubIssue, GitHubComment } from "../../domain/types/issue.ts";

export interface IssueAdapterPort {
  findIssueByNumber(number: number): Promise<GitHubIssue | null>;
  findIssueByTitle(title: string): Promise<GitHubIssue | null>;
  createIssue(title: string, body: string, labels: string[]): Promise<GitHubIssue>;
  updateIssue(issueNumber: number, title: string, body: string): Promise<void>;
  getComments(issueNumber: number): Promise<GitHubComment[]>;
  createComment(issueNumber: number, body: string): Promise<GitHubComment>;
  updateComment(commentId: string, body: string): Promise<void>;
  getCurrentRepo(): Promise<string>;
}

export interface FileAdapterPort {
  readdir(path: string): string[];
  stat(path: string): { isDirectory(): boolean; isFile(): boolean };
  readFile(path: string): string;
  writeFile(path: string, content: string): void;
}
