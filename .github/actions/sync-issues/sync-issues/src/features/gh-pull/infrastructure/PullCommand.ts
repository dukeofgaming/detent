/**
 * PullCommand - Infrastructure Layer
 * Implements the pull subcommand for syncing issues from GitHub to local files
 * Note: gh-pull slice NEVER modifies GitHub - this is read-only
 * (Copied from gh-push - vertical slices do not share code)
 */

import type { Command, CommandContext, CommandResult } from "#application/ports";
import type { IssueFile } from "#domain/types";
import { parseIssueFile } from "#domain/services/ParseIssueFile";
import { GhCliAdapter } from "#adapters/GhCliAdapter";
import { NodeFileAdapter } from "#adapters/NodeFileAdapter";

const ISSUES_DIR = Deno.cwd() + "/docs/issues";

interface Change {
  type: "title" | "body" | "canonical-section" | "new-comment";
  local?: string;
  gh?: string;
  section?: string;
  commentId?: string;
}

export class PullCommand implements Command {
  readonly name = "pull";
  readonly description = "Sync changes from GitHub to local markdown";

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
    let appliedCount = 0;

    for (const issue of issues) {
      if (!issue.issueId) {
        console.log(`Skipping ${issue.title}: no issue ID`);
        continue;
      }

      try {
        const changes = await this.detectChanges(issue, ghAdapter, verbose);
        
        if (changes.length === 0) {
          console.log(`Issue #${issue.issueId}: ${issue.title} - IN_SYNC`);
          continue;
        }

        console.log(`\nIssue #${issue.issueId}: ${issue.title}`);
        console.log(`  Status: UPSTREAM_CHANGED`);
        
        for (const change of changes) {
          console.log(`  - ${change.type}: ${this.summarizeChange(change)}`);
        }

        if (dryRun) {
          console.log(`  [DRY RUN] Would apply ${changes.length} change(s)`);
          continue;
        }

        const applied = await this.promptAndApply(issue, changes, fileAdapter);
        if (applied) {
          appliedCount++;
        }
      } catch (error) {
        hasErrors = true;
        console.error(
          `Error processing ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`,
        );
      }
    }

    if (appliedCount > 0) {
      console.log(`\nApplied changes to ${appliedCount} issue(s).`);
    }

    return {
      success: !hasErrors,
      exitCode: hasErrors ? 1 : 0,
    };
  }

  private async detectChanges(issue: IssueFile, ghAdapter: GhCliAdapter, verbose: boolean): Promise<Change[]> {
    if (!issue.issueId) return [];
    
    const changes: Change[] = [];
    const ghIssue = await ghAdapter.getIssueMetadata(issue.issueId);
    if (!ghIssue) return [];

    if (ghIssue.title !== issue.title) {
      changes.push({ type: "title", local: issue.title, gh: ghIssue.title });
    }

    if (ghIssue.body !== (issue.sections.description || "")) {
      changes.push({ type: "body", local: issue.sections.description, gh: ghIssue.body });
    }

    const ghComments = await ghAdapter.getCommentsMetadata(issue.issueId);
    const canonicalSections = issue.frontmatter?.issue?.canonicalSections || ["tasks", "journal"];

    for (const sectionName of canonicalSections) {
      const sectionKey = sectionName.toLowerCase();
      const trackedId = issue.sectionIds[sectionKey];
      if (!trackedId) continue;

      const ghComment = ghComments.find(c => c.id === trackedId);
      if (ghComment) {
        const localContent = issue.sections[sectionKey] || "";
        if (ghComment.body.trim() !== localContent.trim()) {
          changes.push({ 
            type: "canonical-section", 
            section: sectionName,
            local: localContent.substring(0, 100),
            gh: ghComment.body.substring(0, 100),
            commentId: trackedId,
          });
        }
      }
    }

    if (verbose) {
      console.log(`[pull] Detected ${changes.length} changes for issue #${issue.issueId}`);
    }

    return changes;
  }

  private summarizeChange(change: Change): string {
    switch (change.type) {
      case "title":
        return `"${change.local}" → "${change.gh}"`;
      case "body":
        return "content differs";
      case "canonical-section":
        return `${change.section}: content differs`;
      case "new-comment":
        return "new comment from GitHub";
      default:
        return "";
    }
  }

  private async promptAndApply(issue: IssueFile, changes: Change[], fileAdapter: NodeFileAdapter): Promise<boolean> {
    console.log(`\n  Apply changes? [y/n/a] (a=all, n=skip)`);
    
    const input = await this.readLine();
    const choice = input.trim().toLowerCase();
    
    if (choice === "n" || choice === "skip") {
      console.log(`  Skipped.`);
      return false;
    }
    
    if (choice === "a" || choice === "all") {
      for (const change of changes) {
        await this.applyChange(issue, change, fileAdapter);
      }
      console.log(`  Applied ${changes.length} change(s).`);
      return true;
    }
    
    if (choice === "y" || choice === "yes") {
      for (const change of changes) {
        await this.applyChange(issue, change, fileAdapter);
      }
      console.log(`  Applied ${changes.length} change(s).`);
      return true;
    }

    console.log(`  Invalid choice. Skipping.`);
    return false;
  }

  private async applyChange(issue: IssueFile, change: Change, fileAdapter: NodeFileAdapter): Promise<void> {
    const content = fileAdapter.readFile(issue.filepath);
    let newContent = content;

    if (change.type === "title" && change.gh) {
      newContent = newContent.replace(/^#\s+.+$/m, `# ${change.gh}`);
    }

    if (change.type === "body" && change.gh) {
      const lines = newContent.split("\n");
      const newLines: string[] = [];
      let inDescription = false;
      
      for (const line of lines) {
        if (line.match(/^##\s+Description/i)) {
          inDescription = true;
          newLines.push(line);
          continue;
        }
        if (inDescription && line.match(/^##\s+/)) {
          inDescription = false;
        }
        if (inDescription && !line.startsWith("#") && line.trim()) {
          continue;
        }
        newLines.push(line);
      }
      newContent = newLines.join("\n");
      
      const descMatch = newContent.match(/(##\s+Description\n\n)([\s\S]*?)(?=\n##\s+)/);
      if (descMatch) {
        newContent = newContent.replace(descMatch[2], change.gh || "");
      }
    }

    fileAdapter.writeFile(issue.filepath, newContent);
  }

  private async readLine(): Promise<string> {
    const buf = new Uint8Array(1024);
    const n = await Deno.stdin.read(buf);
    return new TextDecoder().decode(buf.subarray(0, n)).trim();
  }

  help(): string {
    return `
pull - Sync changes from GitHub to local markdown

USAGE
  deno run --allow-all index.ts pull [options]

OPTIONS
  --dry-run    Show what would be done without making changes
  --verbose    Enable verbose logging

NOTES
  This command is read-only for GitHub - it never modifies GitHub data.
  It shows what changed in GitHub and lets you apply changes to local files.
  Like 'git pull', it will block if there are upstream changes.
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