# The SK Programming Language

## Overview

SK is a conceptual programming language designed to handle incomplete, approximate, or partially known information as first-class entities. Unlike traditional languages, SK does **not assume all variables have exact values**. Instead, it tracks uncertainty explicitly throughout calculations, decisions, and control flow.  

The language is designed for **safer, more honest, and analyzable computation** in contexts where data may be noisy, incomplete, or evolving.

## Rust Interpreter

The final version of SK is in the works with a full scale rust interpreter.

```sh
cargo build
cargo run -- examples/test.sk # or ./sk examples.sk
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
let n = 3       // n is exactly 3
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

* **Quiet symbolic values** – similar to symbolic, but if unresolved, printing hides the formula:

```js
quiet volume
volume = side^3
```

* **Constant symbolics** – immutable symbolic expressions:

### 2. Operators

* All arithmetic operators propagate uncertainty:

```js
let a = 2
unknown b // 'Same as' let b = unknown

symbolic z = a + b

print(z)          // → a + b
print(z.resolve()) // → 2 + unknown

b = 3

print(z)          // → 2 + 3
print(z.resolve()) // → 5
```

* Zero propagation and Exponentiation edge cases:

```js
0 * unknown = 0
0 ^ 0 = 1
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

### 3 Symbolic Variables

* Symbolics in SK represent arithmetic expressions over unknown or interval values. They are created automatically whenever an operation involves unknowns or other symbolics.

Creation:

```js
unknown b
let a = 2
symbolic z = a + b // automatically creates a symbolic expression
```

Resolution:

* z itself preserves the symbolic formula (a + b)

* z.resolve() evaluates as much as possible using known operands (2 + unknown)

When all operands are known, z.resolve() returns a concrete known value

```js
b = 3             // not unknown anymore
print(z)          // 2 + 3
print(z.resolve()) // → 5
```

Quiet Symbolics

* Quiet symbolics (quiet) hide the formula if not fully resolvable:

```js
quiet volume
volume = side^3
print(volume) // → unknown if side is unknown
```

This guarantees the symbolic formula will not be altered during program execution.

### 4. Epistemic Control Flow

* Control flow respects uncertainty:

```js
let x = [0..1]

if x > 0.5 -> merge {
    result = "high"
} else {
    result = "low"
}

// Runs both results so in this case result = ["low".."high"]


```

### 5. Functions

* Functions can accept uncertain values and propagate uncertainty:

```js
fn addUp(n) {
    let result = n * (n + 1) / 2
    print(result)
}
```

* Functions operate seamlessly on known, interval, unknown, and symbolic values.

### 6. Constraints (Proposed)

* Future SK syntax may allow constraints on unknown values:

```js
unknown x > 0        // x is unknown but positive
unknown y % 2 == 0   // y is unknown but even
```

## Sumary

SK makes unknown and partially known values first-class citizens. Calculations propagate uncertainty, and control flow respects partial knowledge.

* This allows developers to:
* Write safer programs for uncertain environments
* Track assumptions explicitly
* Produce explanations for derived values
* Reason rigorously about partially known information

SK transforms programming from assuming precision to honestly modeling knowledge and ignorance, enabling robust, intelligent systems.

## Flavortown Hackclub

<https://flavortown.hackclub.com/projects/8834>
