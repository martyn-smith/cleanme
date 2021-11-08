Cleaning Utility
===

*Super-simple cleaning utility*

Rationale
---

Ever get annoyed with build systems - LaTeX I'm particularly looking at you - and bad-mannered programs littering your directories with crap? Of course you have. There's *nothin'* worse than fixing a bug and old, faulty build artifacts hanging around. There's lots of tools to deal with it, but they're usually language- or protocol- specific. I don't want to memorise fifteen different cleaning interfaces.

How it works
---

`Clean` generates a list of targets from a `.clean` file in your home or config directory and, starting at the working directory, recursively:

a) searches its current directory for further `.clean` files, updating its targets,
b) removes matching files.

Glob patterns are supported. Directories are also supported, e.g. `__pycache__` has the same effect as `rm -rf __pycache`.

Needless to say, use with caution. Excepting a directory from cleaning can be achieved by placing an empty `.clean` file in that directory.
