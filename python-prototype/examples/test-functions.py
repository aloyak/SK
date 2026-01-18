from sk import *

# 'Sum Function' starts here ---
def sum_body(args):
    x = args["x"]
    y = args["y"]
    return x + y  # uses the SValue operator overloads

sum_fn = SFunction(["x", "y"], sum_body)
# 'Sum Function' ends here ---

# 'AddUp Function' starts here ---
def addUp_body(args):
    n = args["n"]
    return n * (n + Sknown(1)) / Sknown(2)

addUp_fn = SFunction(["n"], addUp_body)
# 'AddUp Function' ends here ---


a = Sknown(2)
b = Sunknown()

result1 = sum_fn(a, b)
result2 = addUp_fn(b)

print(result1.resolve(), result1.kind)    # if any params are not known totally, it returs a symbolic variable
print(result2.resolve(), result2.kind)
