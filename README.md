Clean
===

*The simple filesystem cleaning utility*

Rationale
---

Ever get annoyed with build systems and bad-mannered programs littering your directories with artifacts? Of course you have. There's *nothin'* worse than old, faulty build artifacts hanging around, *especially* if they interfere with newer builds. Losing literal *gigabytes* of free space due to hidden venvs, and LaTeX's array of \*.synctex.gz and \*.aux files aren't pretty either. 

You might have some language specific tools (pyclean, cargo clean, poetry... sorta has a cleaning utility?), but they're usually not comprehensive enough, cleaning *at most* one project, and sometimes requiring you to know what random-string environment has been created.

This cleans everything.

How it works
---

`Clean` generates a list of targets from a `.cleanme` file in your home or config directory and, starting at the working directory, recursively:

a) searches its current directory for further `.cleanme` files, updating its targets accordingly,
b) removes matching files.

Glob patterns are supported. Directories are also supported, e.g. `__pycache__` has the same effect as `rm -rf __pycache`.

Cautionary notes
---

It's a deletion utility. It's almost certainly a really bad idea to run this as root, for example.

If you want to protect the contents of a directory, simply place an empty `.cleanme` file in it.
