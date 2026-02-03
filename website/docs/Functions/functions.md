# Functions

## User Defined Functions

You can define a custom function using the following syntax, SK will assume the return value based on the return, if there is no return it will return ```none```

```rs
fn myfunc(a, b, c=0) { // 'c' is an optional parameter that defaults to 0
    let raw = a + b

    raw + c // no 'return' keyword, it returs the last value of the block
}

myfunc(1, 2)
// or
myfunc(1, 2, 3)
```

> Recursing is also supported but with limitations!

* All functions are public by default, at least for now, the 'pub' keyword will solve this, please see Imports/imports