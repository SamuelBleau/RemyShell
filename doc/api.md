# RemyShell <-> RemyLang API Specification

This document defines the minimal and stable API exposed by RemyShell
to the embedded RemyLang runtime.

The purpose of this API is to allow RemyLang scripts to interact with
the system **only through explicit and controlled interfaces**.

This API is intentionally minimal and will be extended incrementally.

## Design Principles

- RemyLang is OS-agnostic
- All system interactions are injected by RemyShell
- No implicit global state
- No direct syscall access from the language
- Everything exposed is explicit, inspectable, and testable

## Injected Global Objects

- env      → environment variables
- fs       → filesystem access
- process  → process execution

## Environment API (env)

The `env` object provides controlled access to environment variables.
It is distinct from language-level variables.

Environment variables are always strings.

### env.get

env.get(key: string) -> string?

Returns the value of an environment variable if it exists.
Returns null otherwise.

### env.set

env.set(key: string, value: string)

Sets or overrides an environment variable in the current shell session.
This does not automatically export the variable to child processes.

### env.export

env.export(key: string)

Marks an environment variable as exported, making it visible
to child processes spawned after this call.

### env.list

env.list() -> Map<string, string>

Returns all environment variables visible to the current shell session.

- Environment variables are mutable
- Shadowing language variables is not allowed
- env is the only way to access environment variables

## Filesystem API (fs)

The `fs` object provides minimal filesystem access required by the shell.

### fs.cwd

fs.cwd() -> string

Returns the current working directory.

### fs.cd

fs.cd(path: string)

Changes the current working directory.
Fails if the path does not exist or is not a directory.

### fs.exists

fs.exists(path: string) -> bool

Returns true if the given path exists.

- Path resolution is handled by RemyShell
- No direct file reading or writing is exposed at MVP stage

## Process API (process)

The `process` object allows controlled execution of external commands.

### process.run

process.run(command: string, args: [string]) -> CommandResult

Spawns a new process and waits for its completion.

## CommandResult

CommandResult represents the result of a process execution.

struct CommandResult {
  stdout: string
  stderr: string
  status: int
}

- stdout contains the full standard output
- stderr contains the full error output
- status is the exit code of the process

- stdout and stderr are captured eagerly
- Streaming pipelines are out of scope for MVP

## Error Handling

All API calls may fail.

Failures are represented using RemyLang's native error or Result mechanism.

## Explicitly Out of Scope

The following features are intentionally not part of the MVP API:

- Job control (fg/bg)
- Signals
- Streaming pipelines
- Sandboxing
- Resource limits
- Plugin APIs

## Stability Guarantees

This API is considered stable for Phase 1 and Phase 2.

Breaking changes require:
- documentation update
- explicit migration note

## Summary

This API defines the minimal contract between RemyShell and RemyLang.

It prioritizes:
- clarity
- explicitness
- minimal surface area

Future features will be built on top of this foundation.
