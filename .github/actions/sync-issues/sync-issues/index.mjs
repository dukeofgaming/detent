#!/usr/bin/env node

/**
 * Sync Issues CLI Entry Point
 * Wires together: Domain → Application → Adapter → Infrastructure
 */

import { parseArgs } from 'node:util';

import { SyncIssueUseCase } from './application/usecases/sync-issue.mjs';
import { GhCliAdapter } from './infrastructure/gh-cli/gh-adapter.mjs';
import { NodeFileAdapter } from './infrastructure/gh-cli/file-adapter.mjs';
import { parseIssueFile } from './domain/services/parser.mjs';

const ISSUES_DIR = 'docs/issues';

/**
 * Find all issue files recursively
 * @param {string} dir
 * @param {NodeFileAdapter} fileAdapter
 * @returns {import('./domain/types/issue.mjs').IssueFile[]}
 */
function findIssues(dir, fileAdapter) {
  const issues = [];

  for (const entry of fileAdapter.readdir(dir)) {
    const fullPath = `${dir}/${entry}`;
    const statResult = fileAdapter.stat(fullPath);

    if (statResult.isDirectory()) {
      const subIssues = findIssues(fullPath, fileAdapter);
      issues.push(...subIssues);
    } else if (entry.match(/^#?\d+.*\.md$/)) {
      const content = fileAdapter.readFile(fullPath);
      const parentFolder = dir.split('/').pop();
      issues.push(parseIssueFile(fullPath, content, parentFolder));
    }
  }

  return issues;
}

/**
 * Push command - sync all issues to GitHub
 * @param {Object} options
 * @param {boolean} options.dryRun
 * @param {boolean} options.verbose
 * @returns {Promise<Object>}
 */
async function push(options) {
  const { dryRun, verbose } = options;
  const fileAdapter = new NodeFileAdapter();
  const issueAdapter = new GhCliAdapter();
  const useCase = new SyncIssueUseCase(issueAdapter, fileAdapter);

  console.log('Scanning for issues in', ISSUES_DIR);
  const issues = findIssues(ISSUES_DIR, fileAdapter);
  console.log(`Found ${issues.length} issue(s)\n`);

  const results = { created: 0, updated: 0, skipped: 0 };

  for (const issue of issues) {

    try {
      const result = await useCase.execute(issue, { dryRun, verbose });

      if (result.action === 'created') results.created++;
      else if (result.action === 'updated') results.updated++;
      else results.skipped++;

      console.log();
    } catch (error) {
      console.error(`Error syncing ${issue.title}: ${error instanceof Error ? error.message : 'Unknown error'}`);
      results.skipped++;
    }
  }

  console.log('\nSummary:');
  console.log(`  Created: ${results.created}`);
  console.log(`  Updated: ${results.updated}`);
  console.log(`  Skipped: ${results.skipped}`);

  return results;
}

/**
 * Help command
 */
function help() {
  console.log(`
GitHub Issue Sync

USAGE
  node index.mjs <command> [options]

COMMANDS
  push    Sync issues from docs/issues/ to GitHub
  help    Show this help message

OPTIONS
  --dry-run    Show what would be created without making changes
  --verbose    Enable verbose logging
`);
}

// CLI parsing
const { positionals, values } = parseArgs({
  args: process.argv.slice(2),
  options: {
    'dry-run': { type: 'boolean', default: false },
    verbose: { type: 'boolean', default: false },
  },
  allowPositionals: true,
});

const [command = 'help'] = positionals;

if (command === 'push') {
  push({
    dryRun: values['dry-run'],
    verbose: values.verbose,
  });
} else if (command === 'help') {
  help();
  process.exit(0);
} else {
  console.error(`Unknown command: ${command}`);
  help();
  process.exit(1);
}
