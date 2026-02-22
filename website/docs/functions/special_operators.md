# Special Operators

## Interval Operators

* **IMPORTANT**: These functions were moved to the math library, see Libraries/Math for more information!

SK includes basic functions to work with partially known variables (i.e. intervals)

```rs
intersect(A, B) // returns the intersection of both intervals
union(A, B) // returns the smallest possible interval that contains both

mid(A) // returns the midpoint of the interval, returns number
width(A) // returns max - min, a number
```