# File System (fs)

## The ```fs``` library

```rs
import fs
```

Includes these utilities:

* ```os.open(path, mode)```, opens the file at given path, returns the file id (number starting from 0). Modes are ```r``` for read, ```w```, for writting and ```rw``` or ```r+``` to do both. 
* ```os.close(path)```, closes the file at path or with the file's ID
* ```os.read(id)```, returns the file contents from the file's ID
* ```os.write(id, newContent, append?)```, writes the given content to the file, if append is enabled it will only add it to the bottom of the file

* ```os.exists(path)```, returns true if the given path does exists
* ```os.rename(path, newPath)```, changes the given path to a new one

* ```os.list(path)``` returns a list of the files and dirs inside that path


> Note that the File System Library doesn't work in ```safe``` mode!