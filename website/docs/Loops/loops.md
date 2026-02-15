# Loops

## Loops

SK supports basic loop functionality by using the ```loop``` keyword:

```rs
let n = 10
loop {
    print(n)
    n = n + 1

    if n > 10 {
        break
    } else {
        continue // not needed of course
    }
}
```

* The loop will continue infinitely until the program is terminated or the ```break``` keyword is called
* If the ```continue``` keyword is called, the loop will skip the rest of the block and run a new loop.

### For Loops

* Please see Loops/For