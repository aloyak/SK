class SFunction:
    def __init__(self, param_names, body_fn):
        self.param_names = param_names # List with the names of the parameters
        self.body_fn = body_fn  # Actual python function

    def __call__(self, *args):
        if len(args) != len(self.param_names):
            raise ValueError("Argument count missmatch")
        
        # Map paramaeter name with its value
        arg_map = dict(zip(self.param_names, args))

        # Evaluate the body with these arguments
        result = self.body_fn(arg_map)
        return result
    
# Unknowns propagate naturally through the function body.
# Intervals also propagate and combine automatically.
# Symbolics are preserved when any operand is symbolic or unknown.