from sk import *

a = Sinterval(1,5)
b = Sunknown()

z = a*b

print(z.resolve())

b.setInterval(2,3)

print(z.resolve())