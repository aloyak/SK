# Time

## The ```time``` library

```py
import time
```

Basic time library that includes some simple debugging functions too!

* ```time.now()```, returns the current time in seconds from UNIX epoch
* ```time.format(timestamp, optionalFormat)```, converts any timestamp from UNIX to ```YYYY-MM-DD: HH:MM:SS``` unless a custom format is given (optional)
* ```time.sleep()```, freezes the thread for the given amount of seconds, if an interval is given, it will use the interval's midpoint

### Timer

The time library includes a timer! This creates a new thread that counts the time in seconds till the timer is stopped or the program is terminated.

* ```time.startTimer()``` returns the timer ID, starting from 0
* ```time.stopTimer(ID)``` returns the timer elapsed time since start in seconds, requieres the timer's id