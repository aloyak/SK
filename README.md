<div align="center">
  <img src="skicon.png" alt="SK Logo" width="160" height="160"> 
  <h1 style="font-size: 3rem; margin-top: 10px;">The SK Programming Language</h1>
</div>

* **[SK-Lang Website + web IDE + Documentation](https://sk-lang.vercel.app)**

* **[Crates.io](https://crates.io/crates/sk-lang)**

## Overview

SK is a conceptual programming language designed to handle incomplete, approximate, or partially known information as first-class entities. Unlike traditional languages, SK does **not assume all variables have exact values**. Instead, it tracks uncertainty explicitly throughout calculations, decisions, and control flow.  

The language is designed for **safer, more honest, and analyzable computation** in contexts where data may be noisy, incomplete, or evolving.

## Interpreter

The final version of SK is in the works with a full scale Rust interpreter, this interpreter can run files but also features a REPL mode. For more information on the language features and installation please take a look into the [docs](https://sk-lang.vercel.app).

* Install the SK interpreter by running

```sh
cargo install sk-lang
```

* or download the latest binaries [here!](https://github.com/AlmartDev/SK/releases/latest) (currently Linux only)

> Please consider taking a look to the docs here: [documentation](https://sk-lang.vercel.app)

## Python Prototype

SK started as a Python-based prototype. This is now deprecated.

In order to run the python sk tests:

```sh
python3 sk.py <testname> # only after 'test-', inside ./python-prototype/
```

## License

This project is under the MIT License, please see ```LICENSE``` for more information!

## Core Concepts

### 1. Values

SK supports several kinds of values:

* **Known values** – exact, fully determined numbers or objects:

```rs
let n = 3       // n is 3
```

* **Intervals** – ranges of possible numeric values:

```rs
let temperature = [18..24] // temperature can be any value between 18 and 24
```

* **Unknown values** – undefined, but tracked:

```rs
unknown x
```

* **Symbolic values** – formulas that may depend on unknowns or intervals:

```rs
symbolic area
area = side^2
```

* **Quiet symbolic values** – similar to symbolic, but it keeps the formula hidden

```rs
quiet volume
volume = side^3
```

### 2. Operators

* All arithmetic operators propagate uncertainty:

```rs
let a = 2
unknown b // 'Same as' let b = unknown

symbolic z = a + b

print(z)          // → a + b
print(resolve(z)) // → 2 + unknown

b = 3

print(z)          // → 2 + 3
print(resolve(z)) // → 5
```

* Zero propagation and Exponentiation edge cases:

```rs
0 * unknown = 0
0 ^ 0 = 1 // YES it is, Okay?!
```

* Knowledge Operators:

```rs
let val = [10..20]
let check = val > 15

print(check)            // → partial
print(certain(check))   // → false
print(possible(check))  // → true
print(known(val))       // → false
```

* Interval Operators:

```rs
let A = [0..10]
let B = [5..15]

print(width(A))     // → 10
print(mid(A))       // → 5

print(union(A, B))  // → [0..15]
print(intersect(A, B)) // → [5..10]
```

### 3. Epistemic Control Flow

* Control flow respects uncertainty:

```rs
let x = [0..1]

if x > 0.5 -> merge { // merge is an 'if policy'
    result = "high"
} else {
    result = "low"
}
```

* In order to solve uncertain cases, ifs can have different policies:
  * ```strict``` (Default) Doesn't run any branch
  * ```merge```  Runs both branches
  * ```panic```  Runetime Error

* Scopes:

```rs
let a = 10

{
    let a = 15
    let b = 67
    print(a) // → 15
}

print(a) // → 10

print(b) // Error! 'b' is out of scope
```

### 4. Functions

* Functions can accept uncertain values and propagate uncertainty:

```rs
fn addUp(n) {
    n * (n + 1) / 2 // No 'return' keyword! it returns the last value of the block
}

let result = addUp(50)
```

* Functions operate seamlessly on known, interval, unknown, and symbolic values.

### 5. Basic Built-in functions

* ```print()```
* ```input()```
* ```str()``` converts anything to a string
* ```num()``` converts, when possible, to a number
* ```panic!``` or simply ```panic``` (not recommended), throws a run time error and finishes program execution

### 6. Constraints (Proposed)

* Future SK syntax may allow constraints on unknown values:

```rs
unknown x > 0        // x is unknown but positive
unknown y % 2 == 0   // y is unknown but even
```

## VS Code Extension

SK has a language support extension for Visual Studio Code that includes small refinements to make working with SK easier.

* Download the latest version in [here](https://github.com/AlmartDev/SK/releases/latest) or build the extension yourself inside ```extensions/vscode``` with:

```sh
cd extensions/vscode/
vsce package
```

* Then, inside vscode, go to Extensions (```Ctrl + Shift + X```) and select the option from the menu (```...```) -> ```Install from VSIX...```

## Hackclub

Proud member of HackClub!

<https://flavortown.hackclub.com/projects/8834>

<div align="center">
  <a href="https://flavortown.hackclub.com/projects/8834">
    <img src="https://assets.hackclub.com/flag-standalone.svg" alt="Hack Club" width="150" />
  </a>
</div>

### Inspiration

Took many cool ideas from this other project, so ty! @cyteon
[MODU Programming Language](https://github.com/cyteon/modu)