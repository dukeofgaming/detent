/**
 * GhCliAdapter - GitHub CLI adapter implementation
 */

import type { GitHubIssue, GitHubComment } from "../../../domain/types/index.ts";
import type { IssueAdapterPort } from "../../../adapter/index.ts";
import { runGh } from "./RunGh.ts";
import { nodeIdToNumericId } from "./NodeIdToNumericId.ts";

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

  async createIssue(title: string, body: string, labels: string[]): Promise<GitHubIssue> {
    const args = ["issue", "create", "--title", title, "--body", body];
    for (const label of labels) args.push("--label", label);
    const output = runGh(args);
    const number = parseInt(output.split("/").pop() || "0", 10);
    return { number, title, body, labels };
  }

  async updateIssue(issueNumber: number, title: string, body: string): Promise<void> {
    const args = ["issue", "edit", String(issueNumber)];
    if (title) args.push("--title", title);
    if (body !== undefined) args.push("--body", body);
    runGh(args);
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

  async createComment(issueNumber: number, body: string): Promise<GitHubComment> {
    const output = runGh(["issue", "comment", String(issueNumber), "--body", body]);
    const url = output.trim();
    const id = nodeIdToNumericId(url.split("-").pop() || "");
    return { id, body };
  }

  async updateComment(commentId: string, body: string): Promise<void> {
    const repo = runGh(["repo", "view", "--json", "owner,name", "-q", ".owner.login + \"/\" + .name"]);
    const numericId = nodeIdToNumericId(commentId);
    runGh(["api", `repos/${repo}/issues/comments/${numericId}`, "-X", "PATCH", "--field", `body=${body}`]);
  }
}