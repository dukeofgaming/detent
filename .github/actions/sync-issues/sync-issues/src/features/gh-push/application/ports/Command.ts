/**
 * Command Interface - Application Layer
 * Defines the contract for all CLI subcommands
 */

export interface CommandContext {
  verbose: boolean;
  dryRun: boolean;
}

export interface CommandResult {
  success: boolean;
  message?: string;
  exitCode: number;
}

export interface Command {
  readonly name: string;
  readonly description: string;

  execute(context: CommandContext, args: string[]): Promise<CommandResult>;

  help(): string;
}