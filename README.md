<div align="center">
  <img src="skicon.png" alt="SK Logo" width="160" height="160"> 
  <h1 style="font-size: 3rem; margin-top: 10px;">The SK Programming Language</h1>
</div>

[SK-Lang Website + web IDE](https://sk-lang.vercel.app)

[Crates.io](https://crates.io/crates/sk-lang)

## Overview

SK is a conceptual programming language designed to handle incomplete, approximate, or partially known information as first-class entities. Unlike traditional languages, SK does **not assume all variables have exact values**. Instead, it tracks uncertainty explicitly throughout calculations, decisions, and control flow.  

The language is designed for **safer, more honest, and analyzable computation** in contexts where data may be noisy, incomplete, or evolving.

## Rust Interpreter

The final version of SK is in the works with a full scale Rust interpreter.

```sh
cargo build
cargo run -- examples/test.sk # or ./SK test.sk, inside ./interpreter
```

## Python Prototype

The SK prototype is implemented as a **Python-based prototype**. This allows experimenting with the core concepts of uncertain values, intervals, and symbolic expressions using Python classes and operator overloading. Variables such as `SValue`, `SSymbolic`, and their variants simulate the behavior of the language, while arithmetic operations automatically propagate uncertainty and generate symbolic expressions when operands are unknown or partially known. This approach provides a flexible environment to test and refine the semantics of SK before developing a full compiler or interpreter.

In order to run the python sk tests:

```sh
python3 sk.py <testname> # only after 'test-', inside ./python-prototype/
```

## License

Please contact for license information, still in the works.

## Core Concepts

### 1. Values

SK supports several kinds of values:

* **Known values** – exact, fully determined numbers or objects:

```js
let n = 3       // n is 3
```

* **Intervals** – ranges of possible numeric values:

```js
let temperature = [18..24] // temperature can be any value between 18 and 24
```

* **Unknown values** – undefined, but tracked:

```js
unknown x
```

* **Symbolic values** – formulas that may depend on unknowns or intervals:

```js
symbolic area
area = side^2
```

* **Quiet symbolic values** – similar to symbolic, but it keeps the formula hidden

```js
quiet volume
volume = side^3
```

### 2. Operators

* All arithmetic operators propagate uncertainty:

```js
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

```js
0 * unknown = 0
0 ^ 0 = 1 // YES it is, Okay?!
```

* Knowledge Operators:

```js
let val = [10..20]
let check = val > 15

print(check)            // → partial
print(certain(check))   // → false
print(possible(check))  // → true
print(known(val))       // → false
```

* Interval Operators:

```js
let A = [0..10]
let B = [5..15]

print(width(A))     // → 10
print(mid(A))       // → 5

print(union(A, B))  // → [0..15]
print(intersect(A, B)) // → [5..10]
```

### 3. Epistemic Control Flow

* Control flow respects uncertainty:

```js
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

```js
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

```js
fn addUp(n) {
    let result = n * (n + 1) / 2
    result  // returns are the last value given at end of block
}
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

```js
unknown x > 0        // x is unknown but positive
unknown y % 2 == 0   // y is unknown but even
```

## VS Code Extensions

SK has a language support extension for Visual Studio Code that includes small refinements to make working with SK easier.

* Download the latest version in the Releases page or build the extension yourself inside ```extensions/vscode``` with:

```sh
cd extensions/vscode
vsce package
```

* To install, inside vscode, go to Extensions (```Ctrl + Shift + X```) and select the option from the menu ```Install from VSIX...```

## Hackclub

Proud member of HackClub!

<div align="center">
  <a href="https://flavortown.hackclub.com/projects/8834">
    <img src="https://assets.hackclub.com/flag-standalone.svg" alt="Hack Club" width="150" />
  </a>
</div>

### Inspiration

Took many cool ideas from this other project, ty! @cyteon
[modu language github repo](https://github.com/cyteon/modu)