/**
 * Command Registry - Infrastructure Layer
 * Manages registration and lookup of CLI commands
 */

import type { Command } from "../application/ports/index.ts";

export class CommandRegistry {
  private commands: Map<string, Command> = new Map();

  register(command: Command): void {
    this.commands.set(command.name, command);
  }

  get(name: string): Command | undefined {
    return this.commands.get(name);
  }

  getAll(): Command[] {
    return Array.from(this.commands.values());
  }

  has(name: string): boolean {
    return this.commands.has(name);
  }

  listCommands(): string[] {
    return Array.from(this.commands.keys());
  }
}

export function createDefaultRegistry(): CommandRegistry {
  return new CommandRegistry();
}