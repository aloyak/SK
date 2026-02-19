# Installation

## Installation

> SK can be used through the web IDE: [Open the Web IDE](#page-ide)!

There are two main ways to install the SK interpreter, either from github releases or crates.io

### Option A **(recommended)**: install from crates.io, check the website [here!](https://crates.io/crates/sk-lang)

> Note that this option will isntall not install the latest github release but the current version at crates.io (which is most of the time an older version!)

```sh
$ cargo install sk-lang
```

then use ```SK``` on the terminal to run the SK interpreter

### Option B: download from github releases


Download the latest binary from the github repository at:
* [Latest Release](https://github.com/aloyak/SK/releases)

### Check your Installation

Check if the SK interpreter is installed by running:

```sh
$ SK --version
```

## IDE Extensions

Currently, SK code coloring and snippets are only avaliable for vs code, Download the latest version of the extension here

* [Latest Release](https://github.com/aloyak/SK/releases)

or build the extension yourself
```sh
cd extensions/vscode
vsce package
```

Then, install the ```.vsix``` file at Extensions -> Options (...) -> Install from .vsix