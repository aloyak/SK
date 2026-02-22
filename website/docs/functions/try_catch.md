# Try-Catch

# Try-Catch Statement

SK Supports basic error handling with the usage of the ```try``` keyword followed by a ```catch```, this will try to evaluate the first block and in case of a run time error or a ```panic!``` statement (like a division by 0, for example), it will then evaluate the other block instead, which allows the program to run cautiously and preventing unwanted stops

```rs
let span = [-10..10] // fun fact: 'span' was the original name for SK

try {
    for i in span {
        print(1 / i) // division by 0!
    }
} catch {
    print("Error caught!")
}
```

> Note that the catch block doesn't take any arguments, at least for now

This example does nothing. The first block throws an error which is caught by the second, which has no instrucctions other than finishing the block

```rs
try {
    panic!
} catch {
    continue
}
```