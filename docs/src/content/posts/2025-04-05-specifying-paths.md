---
title: How to specify what to lint
---

One design problem we're wrestling with is how to specify the file names to lint.

It is a chicken-and-egg situation: if there is something bad in the file name, you don't (can't?) use it as a command line parameter.

Our current thoughts:

* Use the current directory (as retrieved by the [rust std::env::fs::current_dir()](https://doc.rust-lang.org/std/env/fn.current_dir.html) function) as the default.
* If you don't want the current directory, you can specify one on the command line, but it should be a directory that you named yourself and is under your direct control.
* Require that everything in the directory be either linted or specifically ignored.

## Paths not taken

We looked at other software that specifies files/directories.  We might use some of them for deciding which rules to apply, but the overall directory will be as above.

* [Github Actions](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#onpushpull_requestpull_request_targetpathspaths-ignore) - an array of either `paths` (which supports `!` nots) or `paths-ignore` rules.
* [eslint](https://eslint.org/docs/latest/use/configure/configuration-files#specifying-files-and-ignores) - arrays of `files` and `ignores`, plus [global ignores](https://eslint.org/docs/latest/use/configure/configuration-files#globally-ignoring-files-with-ignores)
* [prettier](https://prettier.io/docs/cli#file-patterns) - takes files on the command line or uses [fast-glob patterns](https://prettier.io/docs/cli#file-patterns) (also on the command line)
