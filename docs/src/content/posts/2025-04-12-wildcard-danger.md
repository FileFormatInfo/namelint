---
title: Wildcards are dangerous
---

We consider wildcards to be very dangerous from a security standpoint.  It is too easy to have something strange in a filename that causes a wildcard to not match.  This made worse if there are multiple programs that evaluate wildcards: something could match in one and not in the other.

We are explicitly choosing not to support wildcards when specifying top-level paths.  Ideally, you should specify the current directory (with `.`) and every file and directory should be included.

Additionally, we want to check *every* file.  Even hidden and ignored files should obey basic sanity rules.  Thus, we have an `not-linted` rule that will flag every file that doesn't get linted.

