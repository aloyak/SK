# Functions

### Built-in Functions

The SK includes many built-in functions,

## Primitive Functions:
```rs
// variable built-ins, see more in Variables/Primitives
num(x) // Converts, when possible, to a numeric variable
str(x) // Converts to a string
```

## The 'panic!' keyword

```panic``` does also work but it is not recommended

```rs
panic! // Ends the program execution
```

## Knowledge Operators

```rs
known(x) // returns wether a variable is known or not
certain(x) // is the condition certain?
possible(x) // can the condition be possible?
impossible(x) // is the condition impossible?
```

## Interval Operators

```rs
// Interval Operators, please see Functions/Special Operators
intersect(A, B)
union(A, B) 
mid(A)
width(A)
```

## 'Resolve'

```rs
// see more in Variables/Symbolics
resolve(symbolics) // returns the resolved value of a symbolic variable
```