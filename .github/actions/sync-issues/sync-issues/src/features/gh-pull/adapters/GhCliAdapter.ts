/**
 * GhCliAdapter - GitHub CLI adapter implementation (read-only)
 * Adapter that implements Application port
 * Note: gh-pull slice NEVER modifies GitHub - this is read-only
 * (Copied from gh-push - vertical slices do not share code)
 */

import type { GitHubIssue, GitHubComment } from "#domain/types";
import type { IssueAdapterPort, IssueMetadata, CommentMetadata } from "#application/ports";
import { runGh } from "#infrastructure/RunGh";

export class GhCliAdapter implements IssueAdapterPort {
  async findIssueByNumber(number: number): Promise<GitHubIssue | null> {
    try {
      const output = runGh(["issue", "view", String(number), "--json", "number,title,body,labels"]);
      if (!output) return null;
      const data = JSON.parse(output);
      return {
        number: data.number,
        title: data.title,
        body: data.body || "",
        labels: data.labels?.map((l: unknown) => typeof l === "string" ? l : (l as { name: string }).name) || [],
      };
    } catch {
      return null;
    }
  }

  async findIssueByTitle(title: string): Promise<GitHubIssue | null> {
    try {
      const output = runGh(["issue", "list", "--state", "all", "--limit", "100", "--json", "number,title,body,labels"]);
      const issues = JSON.parse(output || "[]");
      const found = issues.find((i: GitHubIssue) => i.title === title);
      if (!found) return null;
      return { number: found.number, title: found.title, body: found.body || "", labels: found.labels || [] };
    } catch {
      return null;
    }
  }

  async getComments(issueNumber: number): Promise<GitHubComment[]> {
    try {
      const repo = runGh(["repo", "view", "--json", "owner,name", "-q", ".owner.login + \"/\" + .name"]);
      const output = runGh(["api", `repos/${repo}/issues/${issueNumber}/comments`, "--jq", "[.[] | {id: (.id | tostring), body: .body}]"]);
      if (!output) return [];
      return JSON.parse(output);
    } catch {
      return [];
    }
  }

  async getIssueMetadata(number: number): Promise<IssueMetadata | null> {
    try {
      const output = runGh([
        "issue", "view", String(number),
        "--json", "number,title,body,updatedAt"
      ]);
      if (!output) return null;
      const data = JSON.parse(output);
      return {
        number: data.number,
        title: data.title,
        body: data.body || "",
        updatedAt: data.updatedAt,
      };
    } catch {
      return null;
    }
  }

  async getCommentsMetadata(issueNumber: number): Promise<CommentMetadata[]> {
    try {
      const repo = runGh(["repo", "view", "--json", "owner,name", "-q", ".owner.login + \"/\" + .name"]);
      const output = runGh([
        "api", `repos/${repo}/issues/${issueNumber}/comments`,
        "--jq", "[.[] | {id: (.id | tostring), body: .body, updatedAt: .updated_at, createdAt: .created_at, author: .user.login}]"
      ]);
      if (!output) return [];
      return JSON.parse(output);
    } catch {
      return [];
    }
  }
}