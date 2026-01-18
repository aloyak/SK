from sk import *

a = Sknown(2)
b = Sunknown()

c = Const(Ssymbolic("mul", [a, b]))
print(c)           # mul(2, unknown)
print(c.resolve()) # mul(2, unknown)

# Attempting mutation should fail
try:
    c.operands = [Sknown(3), Sknown(4)]
except Exception as e:
    print("Mutation prevented:", e)
