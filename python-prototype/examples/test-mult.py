from sk import *

x = Sknown(2)
y = Sinterval(-7,4)

print(x*y)

# Can also be done with product(a,b)  
# remember: 0 * unknown = 0

print(product(SValue(0), Sunknown()))