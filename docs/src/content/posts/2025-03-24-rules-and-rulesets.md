---
title: Rules and RuleSets
---

We have an initial design for how linting works.

## Rules

The most detailed item is a **Rule**.  It consists of either a regular expression or a rust function.

## RuleSets

The next level up is **RuleSet**, which is a set of Rules (aren't we good at naming things!).  RuleSets can include other RuleSets, and can also exclude specific Rules.

There will be a standard library of Rules and RuleSets, and you can define your own.  Well, just your own regex Rules, at least for now.

## Tests

A **Test** is a set of Rules/RuleSets applied to files.  When you run namelint, it will do a series of Tests on the directories you specify.  You specify the list of Tests in a configuration file (`namelint.conf` by default).

Every file must be processed by at least one Test (or specifically ignored).  This means bad files cannot sneak through by avoiding all Tests.  Generally, you will just have a catch-all Test that applies some reasonable security rules.

We (the royal "We", meaning Matt) made [schemas](/schemas.html) for both types of files.

