/**
 * Sync Issue Use Case - Application Layer
 * Orchestrates the syncing of an IssueFile to GitHub
 */

/**
 * @typedef {import('../../domain/types/issue.mjs').IssueFile} IssueFile
 * @typedef {import('../../domain/types/issue.mjs').GitHubIssue} GitHubIssue
 * @typedef {import('../../domain/types/issue.mjs').GitHubComment} GitHubComment
 * @typedef {import('../../domain/types/issue.mjs').SyncResult} SyncResult
 * @typedef {import('../../domain/types/issue.mjs').SyncOptions} SyncOptions
 */

/**
 * Interface for GitHub issue operations
 * @interface IssueAdapter
 */

/**
 * Interface for file system operations
 * @interface FileAdapter
 */

/**
 * Sync Issue Use Case
 */
export class SyncIssueUseCase {
  /**
   * @param {Object} issueAdapter
   * @param {Object} fileAdapter
   */
  constructor(issueAdapter, fileAdapter) {
    this.issueAdapter = issueAdapter;
    this.fileAdapter = fileAdapter;
  }

  /**
   * Execute the sync for a single issue file
   * @param {IssueFile} issueFile
   * @param {SyncOptions} options
   * @returns {Promise<SyncResult>}
   */
  async execute(issueFile, options) {
    const { dryRun, verbose } = options;
    const log = (...args) => verbose && console.log('[sync]', ...args);

    log(`Processing: ${issueFile.title} (GH #${issueFile.issueId})`);

    let existingIssue = null;

    if (issueFile.issueId) {
      existingIssue = await this.issueAdapter.findIssueByNumber(issueFile.issueId);
      if (existingIssue) {
        log(`Found existing issue #${existingIssue.number} by ID`);
      }
    }

    if (!existingIssue) {
      existingIssue = await this.issueAdapter.findIssueByTitle(issueFile.title);
      if (existingIssue) {
        log(`Found existing issue #${existingIssue.number} by title`);
      }
    }

    if (dryRun) {
      if (existingIssue) {
        console.log(`[DRY RUN] Would update issue #${existingIssue.number}: ${issueFile.title}`);
        return { action: 'found', issueNumber: existingIssue.number, commentCount: 0 };
      }
      console.log(`[DRY RUN] Would create issue: ${issueFile.title}`);
      return { action: 'created', issueNumber: null, commentCount: 0 };
    }

    let issueNumber;

    if (existingIssue) {
      issueNumber = existingIssue.number;

      const needsUpdate =
        existingIssue.title !== issueFile.title ||
        existingIssue.body !== (issueFile.sections.description || '');

      if (needsUpdate) {
        log(`Updating issue title/body`);
        await this.issueAdapter.updateIssue(
          issueNumber,
          issueFile.title,
          issueFile.sections.description || ''
        );
      }
    } else {
      const body = issueFile.sections.description || '';
      const labels = issueFile.tags.filter(t => !t.includes(':'));
      const created = await this.issueAdapter.createIssue(issueFile.title, body, labels);
      issueNumber = created.number;
      console.log(`Created issue #${issueNumber}`);
    }

    const commentCount = await this.syncComments(issueFile, issueNumber, log);

    return {
      action: existingIssue ? 'updated' : 'created',
      issueNumber,
      commentCount,
    };
  }

  /**
   * Sync ## Tasks and ## Journal sections as comments
   * @param {IssueFile} issueFile
   * @param {number} issueNumber
   * @param {Function} log
   * @returns {Promise<number>}
   */
  async syncComments(issueFile, issueNumber, log) {
    const existingComments = await this.issueAdapter.getComments(issueNumber);

    const sectionNames = ['tasks', 'journal'];
    let commentCount = 0;
    /** @type {Record<string, string>} */
    const newSectionIds = {};

    for (const sectionName of sectionNames) {
      const sectionKey = sectionName.toLowerCase();
      const content = issueFile.sections[sectionKey];
      if (!content) continue;

      const trackedId = issueFile.sectionIds[sectionKey];
      const existingComment = trackedId
        ? existingComments.find(c => c.id === trackedId)
        : this.findCommentByContent(existingComments, content);

      if (existingComment) {
        if (existingComment.body.trim() === content.trim()) {
          log(`Skipping ${sectionName} - content unchanged`);
          continue;
        }
        log(`Updating comment ${existingComment.id} for ${sectionName}`);
        await this.issueAdapter.updateComment(existingComment.id, content);
        commentCount++;
      } else {
        log(`Creating new comment for ${sectionName}`);
        const result = await this.issueAdapter.createComment(issueNumber, content);
        console.log(`Created comment for ${sectionName}: ${result.id}`);
        newSectionIds[sectionName] = result.id;
        commentCount++;
      }
    }

    if (Object.keys(newSectionIds).length > 0) {
      await this.updateFrontmatterSectionIds(issueFile, newSectionIds);
    }

    return commentCount;
  }

  /**
   * Update the frontmatter with new comment IDs
   * @param {IssueFile} issueFile
   * @param {Record<string, string>} newSectionIds
   */
  async updateFrontmatterSectionIds(issueFile, newSectionIds) {
    try {
      const content = this.fileAdapter.readFile(issueFile.filepath);
      const frontmatterMatch = content.match(/^---\n([\s\S]*?)\n---\n/);
      if (!frontmatterMatch) return;

      let frontmatter = frontmatterMatch[1];
      const lines = frontmatter.split('\n');
      const newLines = [];

      let inSectionsBlock = false;
      for (const line of lines) {
        if (line.match(/^    sections:/)) {
          inSectionsBlock = true;
          newLines.push(line);
          continue;
        }
        if (inSectionsBlock && line.match(/^        - \w+:/)) {
          const sectionName = line.match(/^        - (\w+):/)[1];
          if (newSectionIds[sectionName]) {
            newLines.push(`        - ${sectionName}: ${newSectionIds[sectionName]}`);
          } else {
            newLines.push(line);
          }
          continue;
        }
        if (inSectionsBlock && !line.match(/^\s/)) {
          inSectionsBlock = false;
        }
        if (!inSectionsBlock) {
          newLines.push(line);
        }
      }

      for (const [sectionName, commentId] of Object.entries(newSectionIds)) {
        if (!newLines.some(l => l.includes(`- ${sectionName}:`))) {
          newLines.push(`        - ${sectionName}: ${commentId}`);
        }
      }

      const newFrontmatter = newLines.join('\n').trimEnd();
      const newContent = `---\n${newFrontmatter}\n---\n` + content.slice(frontmatterMatch[0].length);
      this.fileAdapter.writeFile(issueFile.filepath, newContent);
      console.log(`Updated frontmatter with comment IDs in ${issueFile.filepath}`);
    } catch (error) {
      console.error(`Failed to update frontmatter: ${error}`);
    }
  }

  /**
   * Find a comment by content comparison (first 100 chars)
   * @param {GitHubComment[]} comments
   * @param {string} content
   * @returns {GitHubComment | undefined}
   */
  findCommentByContent(comments, content) {
    const normalized = content.trim().slice(0, 100);
    return comments.find(c => c.body.trim().slice(0, 100) === normalized);
  }
}
