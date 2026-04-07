#!/usr/bin/env deno run

/**
 * CLI Entry Point - Infrastructure Layer
 * Bootstraps the command runner and registers available commands
 */

import { CommandRegistry } from "./src/features/gh-push/infrastructure/CommandRegistry.ts";
import { createCommandRunner } from "./src/features/gh-push/infrastructure/CommandRunner.ts";
import { PushCommand } from "./src/features/gh-push/infrastructure/PushCommand.ts";
import { HelpCommand } from "./src/features/gh-push/infrastructure/HelpCommand.ts";

const registry = new CommandRegistry();

registry.register(new PushCommand());
registry.register(new HelpCommand(registry));

const runner = createCommandRunner(registry);
const result = await runner.run(Deno.args);
Deno.exit(result.exitCode);