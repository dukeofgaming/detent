/**
 * Push Command - Infrastructure Layer
 * Implements the push subcommand for syncing issues to GitHub
 */

import type { Command, CommandContext, CommandResult } from "#application/ports";
import type { IssueFile } from "#domain/types";
import { parseIssueFile } from "#domain/services/ParseIssueFile";
import { SyncIssueUseCase } from "#application/usecases/SyncIssueUseCase";
import { GhCliAdapter } from "#adapters/GhCliAdapter";
import { NodeFileAdapter } from "#adapters/NodeFileAdapter";

const ISSUES_DIR = Deno.cwd() + "/docs/issues";

export class PushCommand implements Command {
  readonly name = "push";
  readonly description = "Sync issues from docs/issues/ to GitHub";

  async execute(context: CommandContext, _args: string[]): Promise<CommandResult> {
    const { verbose, dryRun } = context;

    console.log("Scanning for issues in", ISSUES_DIR);

    const fileAdapter = new NodeFileAdapter();

    try {
      fileAdapter.stat(ISSUES_DIR);
    } catch {
      console.log(`No issues directory found at ${ISSUES_DIR}`);
      return { success: true, exitCode: 0 };
    }

    const ghAdapter = new GhCliAdapter();
    const syncUseCase = new SyncIssueUseCase(ghAdapter, fileAdapter);

    const issues = this.findIssues(fileAdapter);
    console.log(`Found ${issues.length} issue(s)\n`);

    let hasErrors = false;
    let hasBlockingChanges = false;

    for (const issue of issues) {
      if (issue.issueId && issue.frontmatter?.issue?.lastSyncedAt) {
        const upstream = await syncUseCase.checkForUpstreamChanges(issue);
        if (upstream?.hasChanges) {
          hasBlockingChanges = true;
          console.error(`⚠️  Cannot push #${issue.issueId}: upstream has newer changes`);
          console.error(`   Last synced: ${issue.frontmatter.issue.lastSyncedAt}`);
          console.error(`   GitHub updated: ${upstream.issueUpdatedAt}`);
          if (upstream.commentsChanged) {
            console.error(`   Canonical comments modified`);
          }
          console.error(`   Run 'pull' to review changes first.\n`);
          continue;
        }
      }

      try {
        if (dryRun) {
          console.log(`[DRY RUN] Would push: ${issue.title} (#${issue.issueId || "new"})`);
          continue;
        }
        const result = await syncUseCase.execute(issue, { verbose, dryRun });
        console.log(`Result: ${result.action} (#${result.issueNumber})`);
      } catch (error) {
        hasErrors = true;
        console.error(
          `Error syncing ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`,
        );
      }
    }

    if (hasBlockingChanges) {
      console.error(`Push blocked: ${issues.filter(i => i.issueId && i.frontmatter?.issue?.lastSyncedAt).length} issue(s) have upstream changes`);
      console.error(`Run 'pull' to review, then push again.`);
    }

    return {
      success: !hasErrors && !hasBlockingChanges,
      exitCode: hasBlockingChanges ? 1 : (hasErrors ? 1 : 0),
    };
  }

  help(): string {
    return `
push - Sync issues from docs/issues/ to GitHub

USAGE
  deno run --allow-all index.ts push [options]

OPTIONS
  --dry-run    Show what would be created without making changes
  --verbose    Enable verbose logging
`;
  }

  private findIssues(adapter: NodeFileAdapter): IssueFile[] {
    const issues: IssueFile[] = [];

    const scan = (dir: string): void => {
      for (const entry of adapter.readdir(dir)) {
        const fullPath = `${dir}/${entry}`;
        const statResult = adapter.stat(fullPath);

        if (statResult.isDirectory()) {
          scan(fullPath);
        } else if (entry.match(/^#?\d+.*\.md$/)) {
          const content = adapter.readFile(fullPath);
          const parentFolder = dir.split("/").pop();
          issues.push(parseIssueFile(fullPath, content, parentFolder));
        }
      }
    };

    scan(ISSUES_DIR);
    return issues;
  }
}