# Units

## The ```units``` Library

```rs
import units
```

This library allows to define variables with either predefined variables or user defined variables as well

By default, the units library includes these:

* `m`, `km`, `cm`, `mm`
* `s`, `min`, `h`
* `kg`, `g`, `mg`
* `L`
* `K`
* `Hz`
* `mol`
* `cd`
* `N`
* `J`
* `W`
* `A`, `Ohm`, `V`
* `C`
* `Pa`

Furthermore, the units library includes a ```units.define()``` functions that allows to create your own units

```rs
units.define("mph", 0.44704 m/s)
```

The units library is used by adding a postfix after variable declaration, note that this only works if the library is imported

```rs
let distance = [48..52] m
let time = 14 s

print("velocity:", distance/time) // result in m/s
```