#!/usr/bin/env deno run

/**
 * CLI Entry Point - Infrastructure Layer
 * Bootstraps the command runner and registers available commands
 * Supports both gh-push and gh-pull vertical slices
 */

import { CommandRegistry as PushRegistry } from "./src/features/gh-push/infrastructure/CommandRegistry.ts";
import { createCommandRunner as createPushRunner } from "./src/features/gh-push/infrastructure/CommandRunner.ts";
import { PushCommand } from "./src/features/gh-push/infrastructure/PushCommand.ts";
import { HelpCommand as PushHelpCommand } from "./src/features/gh-push/infrastructure/HelpCommand.ts";

import { CommandRegistry as PullRegistry } from "./src/features/gh-pull/infrastructure/CommandRegistry.ts";
import { createCommandRunner as createPullRunner } from "./src/features/gh-pull/infrastructure/CommandRunner.ts";
import { PullCommand } from "./src/features/gh-pull/infrastructure/PullCommand.ts";
import { RegisterCommand } from "./src/features/gh-pull/infrastructure/RegisterCommand.ts";
import { HelpCommand as PullHelpCommand } from "./src/features/gh-pull/infrastructure/HelpCommand.ts";

const args = Deno.args;
const primaryCommand = args[0] || "help";

const pushSlice = primaryCommand === "push";
const pullSlice = primaryCommand === "pull" || primaryCommand === "register";

if (pushSlice) {
  const registry = new PushRegistry();
  registry.register(new PushCommand());
  registry.register(new PushHelpCommand(registry));
  const runner = createPushRunner(registry);
  const result = await runner.run(args);
  Deno.exit(result.exitCode);
} else if (pullSlice) {
  const registry = new PullRegistry();
  registry.register(new PullCommand());
  registry.register(new RegisterCommand());
  registry.register(new PullHelpCommand(registry));
  const runner = createPullRunner(registry);
  const result = await runner.run(args);
  Deno.exit(result.exitCode);
} else {
  console.log("GitHub Issue Sync - Available commands:");
  console.log("  push      - Push issues from docs/issues/ to GitHub");
  console.log("  pull      - Pull issues from GitHub to docs/issues/");
  console.log("  register  - Register GitHub references in markdown files");
  console.log("Run 'deno run --allow-all index.ts <command> --help' for more info");
  Deno.exit(0);
}