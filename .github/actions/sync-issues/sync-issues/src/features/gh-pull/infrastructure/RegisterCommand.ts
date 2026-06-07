/**
 * RegisterCommand - Infrastructure Layer
 * Registers GitHub references in markdown files
 * This command modifies local markdown files using GH references
 * Note: gh-pull slice NEVER modifies GitHub - it only reads from GH and writes to local files
 * (Copied from gh-push - vertical slices do not share code)
 */

import type { Command, CommandContext, CommandResult } from "#application/ports";
import type { IssueFile } from "#domain/types";
import { parseIssueFile } from "#domain/services/ParseIssueFile";
import { GhCliAdapter } from "#adapters/GhCliAdapter";
import { NodeFileAdapter } from "#adapters/NodeFileAdapter";

const ISSUES_DIR = Deno.cwd() + "/docs/issues";

interface SyncOptions {
  verbose: boolean;
  dryRun: boolean;
}

export class RegisterCommand implements Command {
  readonly name = "register";
  readonly description = "Register GitHub references in markdown files";

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
    const issues = this.findIssues(fileAdapter);
    console.log(`Found ${issues.length} issue(s)\n`);

    let hasErrors = false;

    for (const issue of issues) {
      try {
        await this.registerReferences(issue, ghAdapter, fileAdapter, { verbose, dryRun });
        console.log(`Registered references for: ${issue.title}`);
      } catch (error) {
        hasErrors = true;
        console.error(
          `Error registering ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`,
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
register - Register GitHub references in markdown files

USAGE
  deno run --allow-all index.ts register [options]

OPTIONS
  --dry-run    Show what would be done without making changes
  --verbose    Enable verbose logging

NOTES
  This command reads from GitHub and writes references to local markdown files.
  It never modifies GitHub - it only reads from GH and updates local files.
`;
  }

  private async registerReferences(
    issue: IssueFile,
    ghAdapter: GhCliAdapter,
    fileAdapter: NodeFileAdapter,
    options: SyncOptions,
  ): Promise<void> {
    const { verbose, dryRun } = options;
    const log = (...args: unknown[]) => verbose && console.log("[register]", ...args);

    log(`Processing: ${issue.title} (GH #${issue.issueId})`);

    if (!issue.issueId) {
      log(`No issue ID in frontmatter, skipping: ${issue.title}`);
      return;
    }

    const existingIssue = await ghAdapter.findIssueByNumber(issue.issueId);
    if (!existingIssue) {
      log(`Issue #${issue.issueId} not found in GitHub`);
      return;
    }

    log(`Found GitHub issue #${existingIssue.number}: ${existingIssue.title}`);

    const comments = await ghAdapter.getComments(existingIssue.number);
    log(`Found ${comments.length} comments`);

    if (dryRun) {
      log(`[DRY RUN] Would update ${issue.filepath} with references`);
      return;
    }

    const currentContent = fileAdapter.readFile(issue.filepath);
    const updatedContent = this.updateMarkdownWithReferences(
      currentContent,
      issue,
      existingIssue,
      comments,
    );

    fileAdapter.writeFile(issue.filepath, updatedContent);
    console.log(`Updated ${issue.filepath} with GitHub references`);
  }

  private updateMarkdownWithReferences(
    content: string,
    issue: IssueFile,
    githubIssue: { number: number; title: string },
    comments: { id: string; body: string }[],
  ): string {
    const lines = content.split("\n");
    const newLines: string[] = [];

    const existingCommentPattern = new RegExp(`<!--\\s*GitHub Issue #${issue.issueId}\\s*-->`);
    const hasIssueComment = existingCommentPattern.test(content);

    for (const line of lines) {
      newLines.push(line);
      
      if (line.match(/^#\s+/) && !hasIssueComment) {
        newLines.push(`<!-- GitHub Issue #${issue.issueId} -->`);
      }
    }

    const hasTitleComment = new RegExp(`<!--\\s*GitHub Title:${issue.issueId}\\s*-->`).test(content);
    if (!hasTitleComment && githubIssue.title) {
      newLines.push(`<!-- GitHub Title:${issue.issueId} ${githubIssue.title} -->`);
    }

    const sectionPatterns = [
      { name: "Tasks", pattern: /^##\s+Tasks/i },
      { name: "Journal", pattern: /^##\s+Journal/i },
      { name: "Notes", pattern: /^##\s+Notes/i },
    ];

    for (const sectionPattern of sectionPatterns) {
      const sectionCommentPattern = new RegExp(`<!--\\s*GitHub Section:${sectionPattern.name}:${issue.issueId}\\s*-->`);
      const hasSectionComment = sectionCommentPattern.test(content);
      
      if (!hasSectionComment) {
        for (let i = 0; i < newLines.length; i++) {
          if (sectionPattern.pattern.test(newLines[i])) {
            const sectionKey = sectionPattern.name.toLowerCase();
            const commentId = issue.sectionIds[sectionKey];
            newLines.splice(i + 1, 0, `<!-- GitHub Section:${sectionPattern.name}:${issue.issueId} ${commentId || ""} -->`);
            break;
          }
        }
      }
    }

    return newLines.join("\n");
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