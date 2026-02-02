# Imports

## Imports

You can use the ```import``` keyword to use functions from libraries or files, based on this syntax:

```rs
import math
import "myOtherFile.sk"

sqrt(4) // from the math library
myOtherFilesFunction("Hey!") // from myOtherFile.sk
```

### Currently in the works

* Note that all imported functions don't need any introduction, but this might be changed (for example: ```math.sqrt()```), current file functions are more important than imported functions, so a user defined ```sqrt()``` function will run rather than the ```math.sqrt()``` function.

* Note that all user defined functions are public to all other files, but this might change in the future