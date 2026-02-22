# Symbolics

## Symbolic variables

SK supports the use of special symbolics variables which propagate uncertainty seamlessly.

```rs
let var = [0..100]

symbolic A = 2 * var + 6
let B = 2 * var + 6

print(A)    // returns ((2 * var) + 6) 
print(resolve(A)) // returns [6..206]

var = 1

print(A)    // returns ((2 * var) + 6) 
print(resolve(A)) // returns 8
print(B) // returns [6..206], it doesn´t update
```

### Quiet Symbolic Variables

* Sometimes, we want to be extra safe and make sure that we never reveal the formula to the user, for this we have the ```quiet``` symbolic type.

* Quiet symbolics don´t need to use the resolve function, doing the resolve of a quiet symbolic will return the same as the quiet symbolic itself

```rs
let var = 1

symbolic A = var + 1
quiet B = var + 1

print(A) // returns (var + 1)
print(B) // returns 2

print(resolve(A)) // returns 2
print(resolve(B)) // same as 'print(B)', returns 2
``` 