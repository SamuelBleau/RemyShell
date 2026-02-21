# RemyShell Configuration Specification

This document defines the configuration system of RemyShell.

The configuration is intentionally split into multiple files,
each with a single, well-defined responsibility.

The goal is to make configuration:
- readable
- maintainable
- predictable
- project-aware


---

## 1. Configuration Principles

## Principles

- One file = one responsibility
- No implicit behavior
- Configuration is explicit and inspectable
- Logic is allowed only where it adds value
- Configuration is code only when necessary

---

## 2. Configuration Directory

## Configuration Directory

RemyShell loads its configuration from:

~/.remyshell/

### Expected structure

```text
~/.remyshell/
├── colors.toml
├── prompt.remy
└── init.remy
```

Each file is optional.
Missing files fall back to default behavior.

---

## 3. `colors.toml` — Theme & Colors

### Responsibility

* Define colors and visual theme
* No logic
* No conditions
* No shell behavior

## colors.toml

### Format

* TOML
* Key-value only

### Example

```toml
background = "#1e1e2e"
foreground = "#cdd6f4"

prompt.cwd    = "#89b4fa"
prompt.branch = "#a6e3a1"
prompt.error  = "#f38ba8"
```

### Rules

- colors.toml must not contain logic
- colors are static values
- colors are referenced by symbolic names

---

## 4. `prompt.remy` — Prompt Layout & Display

### Responsibility

* Define what is displayed in the prompt
* Control layout and ordering
* React to shell context (cwd, git, status)

## prompt.remy

### Language

* Written in RemyLang
* Restricted API
* No side effects on environment or filesystem

### Example

```remy
prompt {
  show cwd
  show git.branch
  show git.status
}
```

### Available Context (MVP)

- cwd
- last_command_status
- git.branch (if in repository)
- git.dirty

### Rules

- prompt.remy must not modify env
- prompt.remy must not execute external commands
- prompt logic must be fast and deterministic

---

## 5. `init.remy` — Startup Logic

### Responsibility

* Shell initialization
* Environment setup
* Aliases
* Conditional logic
* Project detection

## init.remy

### Language

* Full RemyLang
* Access to shell APIs (`env`, `fs`, `process`)

### Example

```remy
env.PATH.append("$HOME/.local/bin")

alias gs = git.status

if project.is_rust() {
  env.RUST_LOG = "debug"
}
```

### Allowed Operations

- environment variable manipulation
- alias definitions
- simple conditional logic
- project detection

### Disallowed Operations (MVP)

- long-running processes
- blocking network calls
- interactive input

---

## 6. Aliases

### Definition

Aliases are defined in `init.remy`.

```remy
alias ll = ls("-la")
alias gs = git.status
```

### Rules

- aliases expand before command execution
- aliases are local to the shell session
- aliases do not override built-in language keywords

---

## 7. Project-Specific Configuration (Future)

## Project Configuration (Future Phase)

Planned but not part of MVP:

```text
.project.remy
```

Purpose:

* Project-scoped environment variables
* Project-specific tasks
* Workspace definition

---

## 8. Loading Order

## Configuration Load Order

1. Load default configuration
2. Load colors.toml
3. Load prompt.remy
4. Load init.remy

Errors in configuration:

* are reported clearly
* do not crash the shell
* may disable the faulty file

---

## 9. Error Handling

## Configuration Errors

Configuration errors must:
- indicate file and line number
- explain what failed
- suggest a fix when possible

Example:

```text
Error in prompt.remy:12
Unknown identifier `git.brnch`
Did you mean `git.branch`?
```

---

## 10. Summary

## Summary

RemyShell configuration is:

- split by responsibility
- explicit
- readable
- safe by default

This design avoids monolithic configuration files
and reduces long-term maintenance cost.
