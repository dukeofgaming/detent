/**
 * Command Runner - Infrastructure Layer
 * Orchestrates CLI command execution
 */

import type { CommandContext, CommandResult } from "../application/ports/index.ts";
import { CliParser, type ParsedArgs } from "./CliParser.ts";
import { CommandRegistry } from "./CommandRegistry.ts";
import { HelpCommand } from "./HelpCommand.ts";

export class CommandRunner {
  private parser: CliParser;
  private registry: CommandRegistry;
  private helpCommand: HelpCommand;

  constructor(registry: CommandRegistry) {
    this.parser = new CliParser();
    this.registry = registry;
    this.helpCommand = new HelpCommand(registry);
  }

  run(args: string[] = Deno.args): Promise<CommandResult> {
    const parsed = this.parser.parse(args);
    return this.executeCommand(parsed);
  }

  private async executeCommand(parsed: ParsedArgs): Promise<CommandResult> {
    if (parsed.showHelp || parsed.remaining.includes("--help") || parsed.remaining.includes("-h")) {
      const command = this.registry.get(parsed.command);
      if (command) {
        console.log(command.help());
        return { success: true, exitCode: 0 };
      }
    }

    const command = this.registry.get(parsed.command);

    if (!command) {
      console.error(`Unknown command: ${parsed.command}`);
      console.log("Run 'deno run --allow-all index.ts help' for usage information");
      return { success: false, exitCode: 1 };
    }

    if (parsed.context.verbose) {
      console.log(`[cli] Executing command: ${command.name}`);
    }

    const result = await command.execute(parsed.context, parsed.remaining);

    if (!result.success && result.message) {
      console.error(result.message);
    }

    return result;
  }

  printHelp(): void {
    this.helpCommand.execute({ verbose: false, dryRun: false }, []);
  }

  printCommandHelp(commandName: string): void {
    const command = this.registry.get(commandName);
    if (command) {
      console.log(command.help());
    } else {
      console.error(`Unknown command: ${commandName}`);
      this.printHelp();
    }
  }
}

export function createCommandRunner(registry: CommandRegistry): CommandRunner {
  return new CommandRunner(registry);
}