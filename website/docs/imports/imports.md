# Imports

## Imports

You can use the ```import``` keyword to use functions from libraries or files, based on this syntax:

```rs
import math
import "myOtherFile.sk"

math.sqrt(4) // from the math library
myOtherFile.myOtherFilesFunction("Hey!") // from myOtherFile.sk
```

> Note that any library included inside an imported file won't be also included to the importer

### Aliases: The ```as``` keyword

Allows to create an alias for an imported library, both for built-ins and files

```rs
import math as TheMathLibrary
import "random.sk" as random
```

### The ```pub``` keyword

In order to allow other files to use your fuctions you need to use the 'pub' keyword, which makes it public

```rs
pub fn yap() {
    print("Hello!")
}

// This is a public function that can be called from other files
```

### The ```units``` library

> Work in progress

This is a special library that allows SK to work with built-in units (```m```, ```m/s```, ```s``` ...) as well as defining your own, this is only implemented for syntax and so working with units will have no effect on the code's behaviour (at least yet)

This is the only library that doesn't include prefixation, everything is always on scope and ready to use

```rs
import units

let velocity = [0..100] m/s
let distance = 650 * 10e3 m // or 650 km
```