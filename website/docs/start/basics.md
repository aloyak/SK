# Basics

## Interpreter Basics

This document explains the usage of the SK interpreter, if you haven't installed it yet, go to Docs / Installation

### Usage:
```sh
$ SK --help

SK - {version}
usage: SK : starts a repl interpreter.
       SK <filename> : runs the file at the given path.
       SK --project <path> : runs 'main.sk' at the given path.
       SK --project new : creates a new project.
       SK --version : shows interpreter's version.
       SK --safe : disables some features for website's IDE security
       SK --help : shows this dialog.
```

**As seen at the top, the SK interpreter has 2 modes:**

* It can interpret any given file using ```SK <filename>```, this will read the file in the given path and display its output. The file doesnt need to have the ```.sk``` extension but it will throw in a warning!
* If only ```SK``` is used, the interpreter will start the REPL mode, which is more simplified and lacks some features from the base interpreter

**Also, you can use:**

* Use ```--version``` to display the interpreter's version
* Use ```--help``` to display the first dialog

### Projects

The SK interpreter also supports working with projects rather than just files

* ```--project <path>``` will run ```main.sk``` on the given project path
* ```--project new``` will prompt you to define the new project's name, this will create a new directory in the current path with the project's name as well as a ```main.sk``` file template with some basic code inside

### Safe Mode

To make the Web IDE safe, The ```--safe``` flag was added, this removes the following features of the language:

* ```os.command()```, disabled so no arbitrary code could be ran and other obvious server attacks
* All the functions of the file system (```fs```) library.

> If you must use these features, download the interpreter! See more information at Start/Installation