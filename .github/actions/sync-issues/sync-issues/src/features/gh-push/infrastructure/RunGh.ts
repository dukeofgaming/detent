/**
 * RunGh - Execute gh CLI command
 */

import { spawnSync } from "node:child_process";

export function runGh(args: string[]): string {
  const result = spawnSync("gh", args, { encoding: "utf-8" });
  if (result.status !== 0 && result.stderr) {
    throw new Error(`gh CLI error: ${result.stderr}`);
  }
  return result.stdout.trim();
}