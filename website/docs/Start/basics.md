# Basics

## Interpreter Basics

This file explains the usage of the SK interpreter, if you haven't installed it yet, go to Docs / Installation

### Usage:
```js
$ SK --help

SK - {version}
usage: SK : starts a repl interpreter.
       SK <filename> : runs the file at the given path.
       SK --version : shows interpreter's version.
       SK --help : shows this dialog.
```

**As seen at the top, the SK interpreter has 2 modes:**

* It can interpret any given file using ```SK <filename>```, this will read the file in the given path and display its output. The file doesnt need to have the ```.sk``` extension but it will throw in a warning!

- If only ```SK``` is used, the interpreter will start the REPL mode, which is more simplified and lacks some features from the base interpreter

**Also, you can use:**

* Use ```--version``` to display the interpreter's version

- Use ```--help``` to display the first dialog