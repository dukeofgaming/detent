/**
 * CLI Parser - Infrastructure Layer
 * Handles argument parsing using node:util
 */

import { parseArgs, type ParseArgsConfig } from "node:util";
import type { CommandContext } from "#application/ports";

export interface ParsedArgs {
  command: string;
  context: CommandContext;
  remaining: string[];
  showHelp: boolean;
}

export interface CliParserOptions {
  globalOptions?: ParseArgsConfig["options"];
  enablePositionals?: boolean;
}

export class CliParser {
  private globalOptions: ParseArgsConfig["options"];

  constructor(options: CliParserOptions = {}) {
    this.globalOptions = {
      "dry-run": { type: "boolean", default: false },
      verbose: { type: "boolean", default: false },
      help: { type: "boolean", default: false },
      ...options.globalOptions,
    };
  }

  parse(args: string[]): ParsedArgs {
    const result = parseArgs({
      args,
      options: this.globalOptions,
      allowPositionals: true,
    });

    const positionals = result.positionals;
    const command = positionals[0] || "help";
    const remaining = positionals.slice(1);

    return {
      command,
      context: {
        verbose: result.values.verbose as boolean ?? false,
        dryRun: result.values["dry-run"] as boolean ?? false,
      },
      remaining,
      showHelp: result.values.help as boolean ?? false,
    };
  }
}