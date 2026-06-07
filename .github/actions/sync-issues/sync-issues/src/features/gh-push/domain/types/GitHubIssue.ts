/**
 * GitHubIssue - Represents a GitHub issue
 */

export interface GitHubIssue {
  number: number;
  title: string;
  body: string;
  labels: string[];
}