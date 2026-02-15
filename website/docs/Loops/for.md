# For Loops

## The For Loop

SK allows for easy iteration with the ```for``` loop, they can use explicit iteration by using an array as an argument or can iterate a variable based on an interval that is used as a range

* Explicit Iteration:

```rs
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let even = []

for value in arr {
    if value % 2 == 0 {
        even.push(value)
    }
}
```

* Variable Iteration:

```rs
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let odd = []

for value in [0..arr.len() - 1] { // Interval from 0 to array's length
    if arr[i] % 2 != 0 {
        odd.push(arr[i])
    }
}
```

> Note that for loops also support the ```break``` and ```continue``` keywords.