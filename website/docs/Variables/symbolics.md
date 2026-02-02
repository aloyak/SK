# Symbolics

## Symbolic variables

SK supports the use of special symbolics variables which propagate uncertainty

```rs
let var = [0..100]

symbolic A = 2 * var + 6
quiet B = 2 * var + 6

print(A)    // returns ((2 * var) + 6) 
print(resolve(A)) // returns [6..206]
print(B) // returns [6..206], same as resolve(B)
```

> TODO: Add more here!