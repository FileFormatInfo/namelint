---
title: "Mandatory Rules"
---

There are a some rules that cannot be disabled.  These rules will cause the file to fail linting and additional lints to be skipped.

## UTF-8 and no nulls

These two rules are mandatory:

1. Must be UTF-8
2. Must not have any null (`0`) bytes

Reasoning:

* If it is encoding in something other than UTF-8, we don't to be in the business of converting encodings.
* If it isn't valid UTF-8 on a system that uses UTF-8 encoding, there then something is really wrong.
* If it has a null byte, again, something is really wrong.

## Rust conversions

Rust is pretty explicit about handling potential errors when converting from paths and directory entries.  These failures can't be disabled.  In theory, the UTF-8 required rule should catch them, but if something slips through, it will still be a lint failure.

## Simultaneous file changes

A variety of problems can occur if you are changing files/directories while namelint is running.  Why would you do that?
