/**
 * Domain Types - Core business entities
 * No external dependencies - pure JavaScript with JSDoc
 */

/**
 * @typedef {Object} IssueFrontmatter
 * @property {'issue' | 'journal'} type
 * @property {string} [title]
 * @property {string} [author]
 * @property {string} [created]
 * @property {string} [updated]
 * @property {Object} [issue]
 * @property {number} [issue.id]
 * @property {string[]} [issue.tags]
 * @property {Record<string, string>} [issue.sections]
 */

/**
 * @typedef {Object} IssueFile
 * @property {string} filepath
 * @property {string} filename
 * @property {number | null} issueId
 * @property {string} title
 * @property {'issue' | 'journal'} type
 * @property {string} [author]
 * @property {string} [created]
 * @property {string} [updated]
 * @property {string[]} tags
 * @property {Record<string, string>} sectionIds
 * @property {Record<string, string>} sections
 */

/**
 * @typedef {Object} GitHubIssue
 * @property {number} number
 * @property {string} title
 * @property {string} body
 * @property {string[]} labels
 */

/**
 * @typedef {Object} GitHubComment
 * @property {string} id
 * @property {string} body
 */

/**
 * @typedef {Object} SyncResult
 * @property {'created' | 'updated' | 'skipped' | 'found'} action
 * @property {number | null} issueNumber
 * @property {number} commentCount
 */

/**
 * @typedef {Object} SyncOptions
 * @property {boolean} dryRun
 * @property {boolean} verbose
 */

/**
 * @typedef {Object} FileStat
 * @property {() => boolean} isDirectory
 * @property {() => boolean} isFile
 */

export const ISSUES_DIR = 'docs/issues';
