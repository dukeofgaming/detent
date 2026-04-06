/**
 * Node.js File Adapter - Concrete implementation using Node.js fs module
 */

import { readdirSync, statSync, readFileSync, writeFileSync } from "node:fs";
import type { FileAdapterPort } from "../../../adapter/ports.ts";

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
