from sk import *

x = SValue(0,3)
print(x)

x.kind = SKind.known
print(x)

x.kind = SKind.unknown
print(x)

x.kind = SKind.interval
print(x)