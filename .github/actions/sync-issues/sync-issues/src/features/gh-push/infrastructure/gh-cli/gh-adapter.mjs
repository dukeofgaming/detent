/**
 * GitHub CLI Adapter - Concrete implementation using gh CLI
 * Infrastructure layer - depends on external gh CLI
 */

import { spawnSync } from 'node:child_process';

/**
 * @typedef {import('../../domain/types/issue.mjs').GitHubIssue} GitHubIssue
 * @typedef {import('../../domain/types/issue.mjs').GitHubComment} GitHubComment
 */

/**
 * Run gh CLI command and return output
 * @param {string[]} args
 * @returns {string}
 */
function runGh(args) {
  const result = spawnSync('gh', args, { encoding: 'utf-8' });
  if (result.status !== 0 && result.stderr) {
    throw new Error(`gh CLI error: ${result.stderr}`);
  }
  return result.stdout.trim();
}

/**
 * Convert node_id (IC_xxx) to numeric ID
 * @param {string} nodeId
 * @returns {string}
 */
function nodeIdToNumericId(nodeId) {
  if (!nodeId.includes('_')) return nodeId;
  const match = nodeId.match(/^IC_kw[A-Za-z0-9]+_?(.+)$/);
  return match ? match[1] : nodeId;
}

/**
 * GitHub CLI Adapter implementation
 */
export class GhCliAdapter {
  constructor() {
    this.repo = null;
  }

  /**
   * @param {number} number
   * @returns {Promise<GitHubIssue | null>}
   */
  async findIssueByNumber(number) {
    try {
      const output = runGh(['issue', 'view', String(number), '--json', 'number,title,body,labels']);
      if (!output) return null;
      const data = JSON.parse(output);
      return {
        number: data.number,
        title: data.title,
        body: data.body || '',
        labels: data.labels?.map(l => typeof l === 'string' ? l : l.name) || [],
      };
    } catch {
      return null;
    }
  }

  /**
   * @param {string} title
   * @returns {Promise<GitHubIssue | null>}
   */
  async findIssueByTitle(title) {
    try {
      const output = runGh(['issue', 'list', '--state', 'all', '--limit', '100', '--json', 'number,title,body,labels']);
      const issues = JSON.parse(output || '[]');
      const found = issues.find(i => i.title === title);
      if (!found) return null;
      return {
        number: found.number,
        title: found.title,
        body: found.body || '',
        labels: found.labels || [],
      };
    } catch {
      return null;
    }
  }

  /**
   * @param {string} title
   * @param {string} body
   * @param {string[]} labels
   * @returns {Promise<GitHubIssue>}
   */
  async createIssue(title, body, labels) {
    const args = ['issue', 'create', '--title', title, '--body', body];
    for (const label of labels) {
      args.push('--label', label);
    }
    const output = runGh(args);
    const url = output;
    const number = parseInt(url.split('/').pop() || '0', 10);
    return { number, title, body, labels };
  }

  /**
   * @param {number} issueNumber
   * @param {string} title
   * @param {string} body
   * @returns {Promise<void>}
   */
  async updateIssue(issueNumber, title, body) {
    const args = ['issue', 'edit', String(issueNumber)];
    if (title) args.push('--title', title);
    if (body !== undefined) args.push('--body', body);
    runGh(args);
  }

  /**
   * @param {number} issueNumber
   * @returns {Promise<GitHubComment[]>}
   */
  async getComments(issueNumber) {
    try {
      const repo = await this.getCurrentRepo();
      const output = runGh(['api', `repos/${repo}/issues/${issueNumber}/comments`, '--jq', '[.[] | {id: (.id | tostring), node_id: .node_id, body: .body}]']);
      if (!output) return [];
      return JSON.parse(output);
    } catch {
      return [];
    }
  }

  /**
   * @param {number} issueNumber
   * @param {string} body
   * @returns {Promise<GitHubComment>}
   */
  async createComment(issueNumber, body) {
    const output = runGh(['issue', 'comment', String(issueNumber), '--body', body]);
    const url = output.trim();
    const id = nodeIdToNumericId(url.split('-').pop() || '');
    return { id, body };
  }

  /**
   * @param {string} commentId
   * @param {string} body
   * @returns {Promise<void>}
   */
  async updateComment(commentId, body) {
    const repo = await this.getCurrentRepo();
    const numericId = nodeIdToNumericId(commentId);
    runGh(['api', `repos/${repo}/issues/comments/${numericId}`, '-X', 'PATCH', '--field', `body=${body}`]);
  }

  /**
   * @returns {Promise<string>}
   */
  async getCurrentRepo() {
    if (this.repo) return this.repo;
    try {
      this.repo = runGh(['repo', 'view', '--json', 'owner,name', '-q', '.owner.login + "/" + .name']);
      return this.repo || 'dukeofgaming/detent';
    } catch {
      this.repo = 'dukeofgaming/detent';
      return this.repo;
    }
  }
}
