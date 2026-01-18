from sk import *

x = Sunknown()          # SValue()
y = Sknown(1)           # SValue(1)
z = Sinterval(-3, 42)   # SValue(-3, 42)

print(x.kind, y.kind, z.kind)

print(Sknown(3)/Sinterval(3,9))
print(Sknown(3)**Sinterval(3,9))
print(Sknown(0)**Sunknown())
print(Sknown(0)/Sunknown())