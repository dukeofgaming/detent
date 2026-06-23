/**
 * SyncIssueUseCase - Application Layer
 * Orchestrates the syncing of an IssueFile from GitHub to local files
 * (Copied from gh-push - vertical slices do not share code)
 * Note: This use case fetches from GitHub but writes to local files only
 */

import type { IssueFile, SyncResult, SyncOptions } from "#domain/types";
import type { IssueAdapterPort, FileAdapterPort } from "#application/ports";

export class SyncIssueUseCase {
  constructor(
    private issueAdapter: IssueAdapterPort,
    private fileAdapter: FileAdapterPort,
  ) {}

  async execute(issueFile: IssueFile, options: SyncOptions): Promise<SyncResult> {
    const { dryRun, verbose } = options;
    const log = (...args: unknown[]) => verbose && console.log("[sync]", ...args);

    log(`Processing: ${issueFile.title} (GH #${issueFile.issueId})`);

    let existingIssue = null;

    if (issueFile.issueId) {
      existingIssue = await this.issueAdapter.findIssueByNumber(issueFile.issueId);
      if (existingIssue) log(`Found existing issue #${existingIssue.number} by ID`);
    }

    if (!existingIssue) {
      existingIssue = await this.issueAdapter.findIssueByTitle(issueFile.title);
      if (existingIssue) log(`Found existing issue #${existingIssue.number} by title`);
    }

    if (dryRun) {
      if (existingIssue) {
        console.log(`[DRY RUN] Would update issue #${existingIssue.number}: ${issueFile.title}`);
        return { action: "found", issueNumber: existingIssue.number, commentCount: 0 };
      }
      console.log(`[DRY RUN] Would create issue: ${issueFile.title}`);
      return { action: "created", issueNumber: null, commentCount: 0 };
    }

    let issueNumber: number;

    if (existingIssue) {
      issueNumber = existingIssue.number;
    } else {
      throw new Error(`Issue not found in GitHub: ${issueFile.title}`);
    }

    const commentCount = await this.syncComments(issueFile, issueNumber, log);

    return {
      action: existingIssue ? "updated" : "found",
      issueNumber,
      commentCount,
    };
  }

  async syncComments(
    _issueFile: IssueFile,
    issueNumber: number,
    log: (...args: unknown[]) => void,
  ): Promise<number> {
    const existingComments = await this.issueAdapter.getComments(issueNumber);
    log(`Found ${existingComments.length} comments for issue #${issueNumber}`);
    return existingComments.length;
  }

  async updateFrontmatterSectionIds(
    _issueFile: IssueFile,
    _newSectionIds: Record<string, string>,
  ): Promise<void> {
    // This would update the markdown file with new section IDs
    // For now, this is a placeholder
  }

  findCommentByContent(
    _comments: { id: string; body: string }[],
    _content: string,
  ): { id: string; body: string } | undefined {
    return undefined;
  }
}