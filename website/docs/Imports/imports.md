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

### The 'as' keyword

Allows to create an alias for an imported library, both for built-ins and files

```rs
import math as TheMathLibrary
import "random.sk" as random
```

* Not currently implemented!

### The 'pub' keyword

Currently, all functions are public to any other file that imports them by default, with the 'pub' keyword, it will allow to choose what functions are kept private to the file that contains the definiton or be usable to any other file.

* Not currently implemented!
