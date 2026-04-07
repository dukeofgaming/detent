/**
 * FileAdapterPort - Interface for file system operations
 */

export interface FileAdapterPort {
  readdir(path: string): string[];
  stat(path: string): { isDirectory(): boolean; isFile(): boolean };
  readFile(path: string): string;
  writeFile(path: string, content: string): void;
}