/**
 * Node.js File Adapter - Concrete implementation using Node.js fs module
 * Infrastructure layer - depends on Node.js built-in modules
 */

import { readdirSync, statSync, readFileSync, writeFileSync } from 'node:fs';

/**
 * Node.js File System Adapter implementation
 */
export class NodeFileAdapter {
  /**
   * @param {string} path
   * @returns {string[]}
   */
  readdir(path) {
    return readdirSync(path);
  }

  /**
   * @param {string} path
   * @returns {{ isDirectory(): boolean; isFile(): boolean }}
   */
  stat(path) {
    return statSync(path);
  }

  /**
   * @param {string} path
   * @returns {string}
   */
  readFile(path) {
    return readFileSync(path, 'utf-8');
  }

  /**
   * @param {string} path
   * @param {string} content
   */
  writeFile(path, content) {
    writeFileSync(path, content, 'utf-8');
  }
}
