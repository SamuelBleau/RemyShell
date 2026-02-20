```remy
env.PATH.append("./bin")

let res = run("ls", ["-l"])
if res.status != 0 {
  print(res.stderr)
}
```

And interactive Shell side:

```sh
ls
cd src
git status
```

Rule :

* Simple commands = simple
* logic = RemyLang
