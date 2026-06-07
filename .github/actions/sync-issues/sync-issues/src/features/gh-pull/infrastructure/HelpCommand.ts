/**
 * Help Command - Infrastructure Layer
 * Displays help information for all commands
 * (Copied from gh-push - vertical slices do not share code)
 */

import type { Command, CommandContext, CommandResult } from "#application/ports";
import { CommandRegistry } from "./CommandRegistry.ts";

export class HelpCommand implements Command {
  readonly name = "help";
  readonly description = "Show help information";

  constructor(private registry: CommandRegistry) {}

  async execute(_context: CommandContext, _args: string[]): Promise<CommandResult> {
    const commands = this.registry.getAll();

    console.log(`
GitHub Issue Sync

USAGE
  deno run --allow-all index.ts <command> [options]

COMMANDS
${commands.map((c) => `  ${c.name.padEnd(12)} ${c.description}`).join("\n")}

GLOBAL OPTIONS
  --dry-run    Show what would be created without making changes
  --verbose    Enable verbose logging
`);

    return { success: true, exitCode: 0 };
  }

  help(): string {
    return `
help - Show help information

USAGE
  deno run --allow-all index.ts help [command]

ARGUMENTS
  command    Optional command name to show help for
`;
  }
}