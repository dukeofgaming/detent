#!/usr/bin/env deno run

/**
 * Sync Issues - CLI Entry Point
 * Uses clean architecture layers from src/features/gh-push/
 */

import { parseArgs } from "node:util";

import { parseIssueFile } from "./src/features/gh-push/domain/services/ParseIssueFile.ts";
import type { IssueFile } from "./src/features/gh-push/domain/types/index.ts";
import { SyncIssueUseCase } from "./src/features/gh-push/application/usecases/SyncIssueUseCase.ts";
import { GhCliAdapter } from "./src/features/gh-push/infrastructure/gh-cli/GhCliAdapter.ts";
import { NodeFileAdapter } from "./src/features/gh-push/infrastructure/gh-cli/NodeFileAdapter.ts";

const ISSUES_DIR = Deno.cwd() + "/docs/issues";

function findIssues(adapter: NodeFileAdapter): IssueFile[] {
  const issues: IssueFile[] = [];

  function scan(dir: string): void {
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
  }

  scan(ISSUES_DIR);
  return issues;
}

async function push(verbose: boolean, dryRun: boolean): Promise<void> {
  console.log("Scanning for issues in", ISSUES_DIR);

  const fileAdapter = new NodeFileAdapter();
  const ghAdapter = new GhCliAdapter();
  const syncUseCase = new SyncIssueUseCase(ghAdapter, fileAdapter);

  const issues = findIssues(fileAdapter);
  console.log(`Found ${issues.length} issue(s)\n`);

  for (const issue of issues) {
    try {
      const result = await syncUseCase.execute(issue, { verbose, dryRun });
      console.log(`Result: ${result.action} (#${result.issueNumber})`);
    } catch (error) {
      console.error(`Error syncing ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`);
    }
  }
}

function help(): void {
  console.log(`
GitHub Issue Sync

USAGE
  deno run --allow-all index.ts <command> [options]

COMMANDS
  push    Sync issues from docs/issues/ to GitHub
  help    Show this help message

OPTIONS
  --dry-run    Show what would be created without making changes
  --verbose    Enable verbose logging
`);
}

const { positionals, values } = parseArgs({
  args: Deno.args,
  options: {
    "dry-run": { type: "boolean", default: false },
    verbose: { type: "boolean", default: false },
  },
  allowPositionals: true,
});

const [command = "help"] = positionals;

if (command === "push") {
  push(values.verbose, values["dry-run"]);
} else if (command === "help") {
  help();
  Deno.exit(0);
} else {
  console.error(`Unknown command: ${command}`);
  help();
  Deno.exit(1);
}