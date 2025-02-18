# orange-cat
A cat clone which colorizes its output according to a config file

## Colorize your log output

Many tools produce logs, not all tools colorize it. It's a pain remembering commands like `grails -Dspring.output.ansi.enabled=always run-app` and they don't always work or bug out with obscure errors. Now you can just pipe your output to orange-cat and it will simply work.

orange-cat is very simple. It searches each line read from its input file (or stdin) for regex expressions defined in its config file. If it finds one, then it colorizes that line according to the rules of the config file.

You can see the default config location, by passing the argument `--show-cfg-path` - it will be created the first time you run orange-cat. It should be reasonably obvious how to alter the file.

```toml
[[rules]]
regex = "ERROR"
foreground_color = "red"

[[rules]]
regex = "WARN"
foreground_color = "magenta"

[[rules]]
regex = "INFO"
foreground_color = "yellow"

[[rules]]
regex = "DEBUG"
foreground_color = "green"
```

The rules accept *regex* expressions, so you can try to get more fancy if you want. The regex should be according to the cargo package [`regex`](https://crates.io/crates/regex), and you can try both foreground and background colors from the [`colored` crate](https://crates.io/crates/colored).

If you pass a config file to orange-cat using the `--cfg-file=` syntax, it will use that instead of the default one.

And of course you can get help using `orange-cat --help`.

## Installation

Currently the only way to get orange-cat is cargo install:

```bash
	cargo install orange-cat
```

You'll need a recent version of rust and cargo, which you can get [here](https://www.rust-lang.org/tools/install).

## About

This was a short project to learn a bit about rust cli apps, I followed [this excellent tutorial](https://rust-cli.github.io/book/index.html). I'm planning to expand it a bit more, but it does what I want it to soooo... yeah, not sure how much brain power I will expend.
