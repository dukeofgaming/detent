/**
 * Push Command - Infrastructure Layer
 * Implements the push subcommand for syncing issues to GitHub
 */

import type { Command, CommandContext, CommandResult } from "../application/ports/index.ts";
import type { IssueFile } from "../domain/types/index.ts";
import { parseIssueFile } from "../domain/services/ParseIssueFile.ts";
import { SyncIssueUseCase } from "../application/usecases/SyncIssueUseCase.ts";
import { GhCliAdapter } from "../adapters/GhCliAdapter.ts";
import { NodeFileAdapter } from "../adapters/NodeFileAdapter.ts";

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

    for (const issue of issues) {
      try {
        const result = await syncUseCase.execute(issue, { verbose, dryRun });
        console.log(`Result: ${result.action} (#${result.issueNumber})`);
      } catch (error) {
        hasErrors = true;
        console.error(
          `Error syncing ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`,
        );
      }
    }

    return {
      success: !hasErrors,
      exitCode: hasErrors ? 1 : 0,
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