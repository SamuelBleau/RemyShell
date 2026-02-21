## Global Overview — Mental Model

RemyShell is made of **three strictly separated layers**:

```text
┌───────────────┐
│   User Input  │
└───────┬───────┘
        ↓
┌──────────────────────┐
│      RemyShell       │  ← REPL, config, env, fs, process
│  (Rust runtime)      │
└───────┬──────────────┘
        ↓ (API calls)
┌──────────────────────┐
│      RemyLang        │  ← parsing, execution, scripts
│  (VM / Interpreter)  │
└───────┬──────────────┘
        ↓ (syscalls)
┌──────────────────────┐
│        OS            │
└──────────────────────┘
```

### Golden Rule

> RemyLang **does not know the OS**
> RemyShell **does not know the language’s internal syntax**

---

## Runtime Initialization (Shell Startup)

```text
Shell start
   │
   ├── Load default config
   │
   ├── Load ~/.remyshell/colors.toml
   │
   ├── Initialize RemyLang runtime
   │     ├── register env object
   │     ├── register fs object
   │     └── register process object
   │
   ├── Load ~/.remyshell/prompt.remy
   │
   ├── Load ~/.remyshell/init.remy
   │
   └── Enter REPL loop
```

### Important Invariant

* The language **exists before** `init.remy`
* The shell **injects bindings before execution**

---

## REPL Loop — High-Level Flow

```text
┌────────────────────────────┐
│ Render prompt (prompt.remy)│
└────────────┬───────────────┘
             ↓
┌────────────────────────────┐
│ Read user input (string)   │
└────────────┬───────────────┘
             ↓
┌────────────────────────────┐
│ Parse input                │
│ ├─ shell command?          │
│ └─ RemyLang expression?    │
└────────────┬───────────────┘
             ↓
      ┌──────┴──────┐
      │             │
      ↓             ↓
Shell command     RemyLang code
(expand aliases)  (evaluate)
      │             │
      ↓             ↓
process.run       Language VM
      │             │
      └──────┬──────┘
             ↓
┌────────────────────────────┐
│ Capture result / error     │
└────────────┬───────────────┘
             ↓
┌────────────────────────────┐
│ Update shell state         │
│ (last status, cwd, env)    │
└────────────┬───────────────┘
             ↓
Loop
```

---

## Command Execution Path

### External Command

```text
User: ls -la

Input → shell parser
      → alias expansion
      → process.run("ls", ["-la"])
      → OS spawn
      → wait
      → CommandResult
      → display stdout/stderr
```

### RemyLang Command

```text
User: let x = run("ls", [])

Input → detect language
      → pass string to RemyLang
      → VM executes
      → calls process.run internally
      → returns CommandResult
      → print if needed
```

---

## Prompt Rendering Flow

```text
Before each input:

Shell →
  call RemyLang with prompt.remy
    provide context:
      - cwd
      - git info
      - last_command_status

RemyLang →
  returns a string (or structured prompt)

Shell →
  renders with colors
```

### Constraint

* Prompt must be **fast**
* No I/O inside the prompt
* Cached Git info is allowed

---

## Environment Variable Flow

```text
init.remy:
  env.set("FOO", "bar")
  env.export("FOO")

Shell:
  updates internal env map

process.run:
  inherits exported env only
```

---

## Error Propagation

```text
OS error
  ↓
Rust error
  ↓
Shell error
  ↓
RemyLang error
  ↓
User-friendly message
```

### Rule

> Errors bubble **up**, never sideways.

---

## What the Runtime Does NOT Do (MVP)

```text
- No async execution
- No job control
- No signal handling
- No pipelines
- No background tasks
```

All of this is for **future extensions**.
