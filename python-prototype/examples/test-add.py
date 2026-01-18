from sk import *

print(Sinterval(1,2) - Sinterval(1,2))

arr = [
    Sknown(2),
    Sunknown(),
    Sinterval(8,16),
    Sknown(8),
    Sinterval(-1,1),
    Sknown(1),
    Sknown(2)
]

print("Addition")
for i, value in enumerate(arr):
    if i + 1 < len(arr):
        print(f"{value} + {arr[i+1]} = {value + arr[i+1]}")

# Can also be done with add(a,b)
print(f"\n{add(arr[0], arr[2])}\n")

print("Substraction")
for i, value in enumerate(arr):
    if i + 1 < len(arr):
        print(f"{value} - {arr[i+1]} = {value - arr[i+1]}")

# Can only be done with add, so a-b = add(a, -b)
print(f"\n{add(arr[0], -arr[2])}\n")