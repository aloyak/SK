from sk import *

temp = Sinterval(22, 23)
threshold = Sknown(22.5)

def activate_cooler():
    print("cooler ON")

def activate_heater():
    print("heater ON")

# Default policy: run_both
epistemic_if(temp > threshold, activate_cooler, activate_heater)
# Output:
# cooler ON
# heater ON

# Strict policy: don't run anything
epistemic_if(temp > threshold, activate_cooler, activate_heater)
# Output: (nothing)

# Fail-on-partial policy: raise exception
try:
    epistemic_if(temp > threshold, activate_cooler, activate_heater)
except ValueError as e:
    print(e)
# Output:
# Partial or unknown condition: [0..1]
