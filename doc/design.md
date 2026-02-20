### RemyLang (core language)

Responsible for:

* local variables
* types
* functions
* modules
* control flow
* errors (`Result`, exceptions)
* script execution

Does **NOT** handle:

* OS management
* direct filesystem access
* fork / exec
* environment management
* signals

RemyLang is **OS-agnostic**.

---

### RemyShell (runtime & terminal)

Responsible for:

* REPL
* environment (`PATH`, `HOME`, etc.)
* filesystem (cwd, cd)
* external command execution
* job control (later)
* user configuration
* project / git integration

The shell **injects** the real world into the language.

This separation is **non-negotiable**.
