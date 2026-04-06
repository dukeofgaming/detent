#!/usr/bin/env deno run

/**
 * Sync Issues CLI Entry Point
 * Wires together: Domain → Application → Adapter → Infrastructure
 */

import { parseArgs } from "node:util";

import { SyncIssueUseCase } from "./src/features/gh-push/application/usecases/sync-issue.ts";
import { GhCliAdapter } from "./src/features/gh-push/infrastructure/gh-cli/adapter.ts";
import { NodeFileAdapter } from "./src/features/gh-push/infrastructure/gh-cli/file-adapter.ts";
import { parseIssueFile } from "./src/features/gh-push/domain/services/parser.ts";

const ISSUES_DIR = "docs/issues";

function findIssues(dir: string, fileAdapter: NodeFileAdapter) {
  const issues: ReturnType<typeof parseIssueFile>[] = [];

  for (const entry of fileAdapter.readdir(dir)) {
    const fullPath = `${dir}/${entry}`;
    const statResult = fileAdapter.stat(fullPath);

    if (statResult.isDirectory()) {
      issues.push(...findIssues(fullPath, fileAdapter));
    } else if (entry.match(/^#?\d+.*\.md$/)) {
      const content = fileAdapter.readFile(fullPath);
      const parentFolder = dir.split("/").pop();
      issues.push(parseIssueFile(fullPath, content, parentFolder));
    }
  }

  return issues;
}

async function push(options: { dryRun: boolean; verbose: boolean }) {
  const { dryRun, verbose } = options;
  const fileAdapter = new NodeFileAdapter();
  const issueAdapter = new GhCliAdapter();
  const useCase = new SyncIssueUseCase(issueAdapter, fileAdapter);

  console.log("Scanning for issues in", ISSUES_DIR);
  const issues = findIssues(ISSUES_DIR, fileAdapter);
  console.log(`Found ${issues.length} issue(s)\n`);

  const results = { created: 0, updated: 0, skipped: 0 };

  for (const issue of issues) {
    try {
      const result = await useCase.execute(issue, { dryRun, verbose });

      if (result.action === "created") results.created++;
      else if (result.action === "updated") results.updated++;
      else results.skipped++;

      console.log();
    } catch (error) {
      console.error(
        `Error syncing ${issue.title}: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
      results.skipped++;
    }
  }

  console.log("\nSummary:");
  console.log(`  Created: ${results.created}`);
  console.log(`  Updated: ${results.updated}`);
  console.log(`  Skipped: ${results.skipped}`);

  return results;
}

function help() {
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
  push({
    dryRun: values["dry-run"],
    verbose: values.verbose,
  });
} else if (command === "help") {
  help();
  Deno.exit(0);
} else {
  console.error(`Unknown command: ${command}`);
  help();
  Deno.exit(1);
}
