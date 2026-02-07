# Basic Variables

## Primitive Variables

SK features special variables based on how certain their information is, for this reason, any variable can be totally known (numbers, strings, bools), partiall known (i.e the interval type), or unknown

* These are the primitive functions:
```rs
let number = 3 // numeral variable, stored as a 64-bit floating-point
let string = "Welcome to SK!" // supports concatenation with the '+' operator

let interval = [1..2] // interval, represents a partially known variable, from 1 to 2

unknown unk // unknown value, good for initialization
let unk = unknown // can also be defined with this

let boolean = partial // booleans can be true, false and partial (uncertain condition)
```

* Note that intervals may support non-numeric variables in the future, like: ```let interval = ["low".."high"]```

> In the future, the array type might be added

* The variables come with primitive functions to convert into others, when possible
```rs
let number = 42
let string_from_number = str(number) // or str(42)

let string = "42"
let number_from_string = num(string) // or num("42")
```

> More variable primitive functions might be added in the future

### Concatenation
Strings support concatenation using the '+' operator
```rs
let name = "John SK"
print("Hello, " + name + "!")
```

### Increment and Decrement Operators

Just like in C++ you can use the Increment and Decrement Operators

```rs
let n = 23

n++ // 24
n-- // 23

// Same as
n = n + 1
n = n - 1
```