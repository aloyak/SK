from sk import *

print(Sknown(0) ** Sknown(0))        # 1
print(Sknown(0) ** Sknown(5))        # 0
print(Sknown(2) ** Sknown(3))        # 8
print(Sinterval(2,4) ** Sknown(2))   # [4..16]
