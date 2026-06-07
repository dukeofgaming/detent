/**
 * FileAdapterPort - Port for file system operations
 * (Copied from gh-push - vertical slices do not share code)
 */

export interface FileAdapterPort {
  readdir(path: string): string[];
  stat(path: string): { isDirectory(): boolean; isFile(): boolean };
  readFile(path: string): string;
  writeFile(path: string, content: string): void;
}