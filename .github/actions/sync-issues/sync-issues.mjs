#!/usr/bin/env node

import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { join, relative, basename } from 'node:path';
import { spawnSync } from 'node:child_process';

const { parseArgs } = await import('node:util');

const ISSUES_DIR = 'docs/issues';

function runGh(args) {
  const result = spawnSync('gh', args, { encoding: 'utf-8' });
  if (result.status !== 0 && result.stderr) {
    throw new Error(`gh CLI error: ${result.stderr}`);
  }
  return result.stdout.trim();
}

function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---\n/);
  if (!match) return null;
  
  const yaml = match[1];
  const result = {};
  
  for (const line of yaml.split('\n')) {
    const colonIdx = line.indexOf(':');
    if (colonIdx === -1) continue;
    
    const key = line.slice(0, colonIdx).trim();
    let value = line.slice(colonIdx + 1).trim();
    
    if (value.startsWith('[')) {
      result[key] = value.replace(/[\[\]]/g, '').split(',').map(v => v.trim());
    } else if (value) {
      result[key] = value.replace(/^["']|["']$/g, '');
    }
  }
  
  return result;
}

function extractSections(content) {
  const sections = {};
  const lines = content.split('\n');
  let currentSection = null;
  let currentContent = [];
  
  for (const line of lines) {
    const h2Match = line.match(/^##\s+(.+)/);
    if (h2Match) {
      if (currentSection) {
        sections[currentSection] = currentContent.join('\n').trim();
      }
      currentSection = h2Match[1].toLowerCase();
      currentContent = [];
    } else if (currentSection) {
      currentContent.push(line);
    }
  }
  
  if (currentSection) {
    sections[currentSection] = currentContent.join('\n').trim();
  }
  
  return sections;
}

function parseIssueFile(filepath) {
  const content = readFileSync(filepath, 'utf-8');
  const frontmatter = parseFrontmatter(content);
  const sections = extractSections(content);
  
  const filename = basename(filepath, '.md');
  const title = frontmatter?.title || 
    (content.match(/^#\s+(.+)/m)?.[1]) || 
    filename;
  
  return {
    filepath,
    title,
    filename,
    type: frontmatter?.type || 'issue',
    author: frontmatter?.author,
    created: frontmatter?.created,
    updated: frontmatter?.updated,
    tags: frontmatter?.tags || [],
    sections,
    sectionIds: frontmatter?.sections || {},
  };
}

function findIssuesRecursive(dir) {
  const issues = [];
  
  for (const entry of readdirSync(dir)) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);
    
    if (stat.isDirectory()) {
      issues.push(...findIssuesRecursive(fullPath));
    } else if (entry.match(/^#?\d+\.md$/)) {
      issues.push(parseIssueFile(fullPath));
    }
  }
  
  return issues;
}

function findExistingIssue(title) {
  try {
    const output = runGh(['issue', 'list', '--state', 'all', '--limit', '100', '--json', 'number,title']);
    const issues = JSON.parse(output || '[]');
    return issues.find(i => i.title === title);
  } catch {
    return null;
  }
}

function createIssue(issue) {
  const body = issue.sections.description || '';
  const tags = issue.tags.filter(t => !t.includes(':'));
  
  const args = [
    'issue', 'create',
    '--title', issue.title,
    '--body', body,
  ];
  
  for (const tag of tags) {
    args.push('--label', tag);
  }
  
  const output = runGh(args);
  const url = output;
  const number = parseInt(url.split('/').pop(), 10);
  
  return { number, url, body };
}

function createComment(issueNumber, body) {
  const output = runGh(['issue', 'comment', String(issueNumber), '--body', body]);
  const url = output.trim();
  const id = url.split('-').pop();
  return { id, url };
}

function updateComment(repo, commentId, body) {
  const numericId = commentId.includes('_') 
    ? commentId.replace(/^IC_kw[A-Za-z0-9]+/, '').replace(/^_/, '')
    : commentId;
  
  const args = [
    'api',
    `repos/${repo}/issues/comments/${numericId}`,
    '-X', 'PATCH',
    '--field', `body=${body}`
  ];
  return runGh(args);
}

function getCommentsForIssue(issueNumber) {
  try {
    const output = runGh(['api', `repos/${getCurrentRepo()}/issues/${issueNumber}/comments`, '--jq', '[.[] | {id: (.id | tostring), node_id: .node_id, body: .body}]']);
    if (!output) return [];
    return JSON.parse(output);
  } catch {
    return [];
  }
}

function getCommentNumericId(nodeId) {
  try {
    const output = runGh(['api', `repos/${getCurrentRepo()}/issues/comments/${nodeId}`, '--jq', '.id']);
    return output;
  } catch {
    return null;
  }
}

function findCommentByContent(comments, content) {
  const normalized = content.trim().slice(0, 100);
  return comments.find(c => c.body.trim().slice(0, 100) === normalized);
}

function getCurrentRepo() {
  try {
    const output = runGh(['repo', 'view', '--json', 'owner,name', '-q', '.owner.login + "/" + .name']);
    return output || 'dukeofgaming/detent';
  } catch {
    return 'dukeofgaming/detent';
  }
}

function syncIssueToGitHub(issue, options = {}) {
  const { dryRun = false, verbose = false } = options;
  const log = (...args) => verbose && console.log('[sync]', ...args);
  
  log(`Processing: ${issue.title}`);
  
  const existing = findExistingIssue(issue.title);
  
  if (existing) {
    log(`Found existing issue #${existing.number}`);
    
    if (dryRun) {
      console.log(`[DRY RUN] Would update issue #${existing.number}: ${issue.title}`);
      return { action: 'found', number: existing.number };
    }
    
    const existingComments = getCommentsForIssue(existing.number);
    const sectionNames = ['tasks', 'journal', 'notes', 'experiments log'];
    const repo = getCurrentRepo();
    
    for (const sectionName of sectionNames) {
      const sectionKey = sectionName.toLowerCase();
      const content = issue.sections[sectionKey];
      if (!content) continue;
      
      const trackedId = issue.sectionIds[sectionKey];
      const existingComment = trackedId 
        ? existingComments.find(c => c.id === trackedId)
        : findCommentByContent(existingComments, content);
      
      if (existingComment) {
        if (existingComment.body.trim() === content.trim()) {
          log(`Skipping ${sectionName} - content unchanged`);
          continue;
        }
        log(`Updating existing comment ${existingComment.id} for ${sectionName}`);
        updateComment(repo, existingComment.id, content);
      } else {
        log(`Creating new comment for ${sectionName}`);
        const result = createComment(existing.number, content);
        console.log(`Created comment for ${sectionName}: ${result.url}`);
      }
    }
    
    return { action: 'updated', number: existing.number };
  } else {
    log('Creating new issue');
    
    if (dryRun) {
      console.log(`[DRY RUN] Would create issue: ${issue.title}`);
      return { action: 'created', number: null };
    }
    
    const { number, url, body } = createIssue(issue);
    console.log(`Created issue #${number}: ${url}`);
    
    const sectionNames = ['tasks', 'journal', 'notes', 'experiments log'];
    
    for (const sectionName of sectionNames) {
      const sectionKey = sectionName.toLowerCase();
      const content = issue.sections[sectionKey];
      if (!content) continue;
      
      log(`Creating comment for ${sectionName}`);
      const result = createComment(number, content);
      console.log(`Created comment for ${sectionName}: ${result.url}`);
    }
    
    return { action: 'created', number };
  }
}

function push(options = {}) {
  console.log('Scanning for issues in', ISSUES_DIR);
  
  const issues = findIssuesRecursive(ISSUES_DIR);
  console.log(`Found ${issues.length} issue(s)\n`);
  
  const results = { created: 0, updated: 0, skipped: 0 };
  
  for (const issue of issues) {
    if (issue.type !== 'issue') continue;
    
    try {
      const result = syncIssueToGitHub(issue, options);
      
      if (result.action === 'created') results.created++;
      else if (result.action === 'updated') results.updated++;
      else results.skipped++;
      
      console.log();
    } catch (error) {
      console.error(`Error syncing ${issue.title}: ${error.message}`);
      results.skipped++;
    }
  }
  
  console.log('\nSummary:');
  console.log(`  Created: ${results.created}`);
  console.log(`  Updated: ${results.updated}`);
  console.log(`  Skipped: ${results.skipped}`);
  
  return results;
}

function help() {
  console.log(`
GitHub Issue Sync

USAGE
  node sync-issues.mjs <command> [options]

COMMANDS
  push    Sync issues from docs/issues/ to GitHub
  help    Show this help message

OPTIONS
  --dry-run    Show what would be created without making changes
  --verbose    Enable verbose logging
`);
}

const commands = { push, help };

const { positionals, values } = parseArgs({
  args: process.argv.slice(2),
  options: {
    'dry-run': { type: 'boolean', default: false },
    verbose: { type: 'boolean', default: false },
  },
  allowPositionals: true,
});

const [command = 'help', ...rest] = positionals;
const handler = commands[command];

if (!handler) {
  console.error(`Unknown command: ${command}`);
  help();
  process.exit(1);
}

const options = {
  dryRun: values['dry-run'],
  verbose: values.verbose,
};

handler(options);

if (command === 'help') process.exit(0);
