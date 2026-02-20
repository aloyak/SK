# HTTP

## The ```http``` library

```py
import http
```

Includes these utilities:

* ```http.get(url)```, returns the body of the response of the given url as a json string, consider using the json lib to parse it (libraries/json)
* ```http.post(url, body)```, posts the given body to url and returns response


