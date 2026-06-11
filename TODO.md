# To Do


## Work

- [ ] directory walker
- [ ] object that has: function, parameter slug, default on/off (or value), short description, long description
- [ ] unit tests for each rule

## Filters

Which things to test

- [ ] type: file or directory or all
- [ ] extensions: only files with specific extension(s)
- [ ] whitelist: allow specific names

## Rules

- [ ] no null bytes
- [ ] utf8
- [ ] no untrimmed (=no leading/trailing whitespace)
- [ ] no windows reserved names
- [ ] shell-safe: no chars that need shell escapes
- [ ] internal whitespace: none | only single spaces | only spaces
- [ ] no dotfiles
- [ ] only posix safe chars
- [ ] url safe
- [ ] url component safe
- [ ] punctuation: singledot,dashunderscore,most
- [ ] case: upper/lower/camel/snake/kebab/pascal/...
- [ ] extension case: upper/lower/any/exact
- [ ] extension rule: only ascii
- [ ] extensions: all/web/mime/common/[list]/...
