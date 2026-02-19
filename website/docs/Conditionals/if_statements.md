# If Statements

## If Statements

Normal if statements are included but with a small twist, heres the normal syntax:

```rs
let var = 1

if var > 2 {
    print("Branch A")
} elif var == 2 {
    print("Branch B")
} else {
    print("Branch C")
}

// can also be inline no problem

if !(var == 5) { print("var is not 5") }
```

### Special Cases

In case of uncertanty, SK introduces ```If Policies```

* Example uncertain case:
```rs
let condition = [10..20] > 15 // returns partial
```

* ```strict```, in case of uncertanty (i.e. condition is partial), it does not run any branch
* ```merge```, this runs both branches
* ```panic```, and this doesn't run any branch and rises an exception that terminates the program execution

In order to select the policiy, we use the '```->```' operator. Note that ```strict``` is the default policy if none is given

```rs
let condition = [10..20] > 15

if condition -> merge {
    print("A")
} else {
    print("B")
}

// This program runs both branches and thus returns 'A' and 'B'
```