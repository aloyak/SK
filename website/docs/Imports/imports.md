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