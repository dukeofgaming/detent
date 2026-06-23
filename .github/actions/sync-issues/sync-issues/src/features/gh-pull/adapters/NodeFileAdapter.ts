/**
 * NodeFileAdapter - Node.js file system adapter implementation
 * Adapter that implements Application port
 * (Copied from gh-push - vertical slices do not share code)
 */

import { readdirSync, statSync, readFileSync, writeFileSync } from "node:fs";
import type { FileAdapterPort } from "#application/ports";

export class NodeFileAdapter implements FileAdapterPort {
  readdir(path: string): string[] {
    return readdirSync(path);
  }

  stat(path: string): { isDirectory(): boolean; isFile(): boolean } {
    return statSync(path);
  }

  readFile(path: string): string {
    return readFileSync(path, "utf-8");
  }

  writeFile(path: string, content: string): void {
    writeFileSync(path, content, "utf-8");
  }
}