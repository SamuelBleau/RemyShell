#### `env`

```remy
env.get(key: string) -> string?
env.set(key: string, value: string)
env.export(key: string)
env.list() -> Map<string, string>
```

---

#### `fs`

```remy
fs.cwd() -> string
fs.cd(path: string)
fs.exists(path: string) -> bool
```

---

#### `process`

```remy
run(cmd: string, args: [string]) -> CommandResult
```

---

#### `CommandResult`

```remy
struct CommandResult {
  stdout: string
  stderr: string
  status: int
}
```
