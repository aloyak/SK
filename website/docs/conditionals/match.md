# Match Statement

## Match Statement

In order to remove redundant or nested if-statements SK features the ```match``` statement, based on the Rust implementation

```rs
let num = 2

match num {
    1 => panic!
    any => print("Number is not 1!")
}
```

Note the ```any``` keyword to define a 'default' behaviour