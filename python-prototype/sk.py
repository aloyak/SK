import os
import sys

if len(sys.argv) > 1:
    if sys.argv[1] == "all":
        examples = [f[:-3] for f in os.listdir("examples") if f.startswith("test-") and f.endswith(".py")]
    else:
        examples = [f"test-{sys.argv[1]}"]
else:
    examples = ["test-symbolics"]

# ===========
clearOnStart = True
# ===========

if clearOnStart: os.system('cls' if os.name=='nt' else 'clear')
for i, filename in enumerate(examples):
    print(f"\n#{i+1} {filename} {'='*20}\n")
    os.system(f"python3 -m examples.{filename}")