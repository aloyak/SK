from sk import *

a = Sknown(2)
b = Sunknown()
c = Sinterval(-3,6)

print(impossible(a > b))
print(known(c))