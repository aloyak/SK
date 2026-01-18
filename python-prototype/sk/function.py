class SFunction:
    def __init__(self, param_names, body_fn):
        self.param_names = param_names # List with the names of the parameters
        self.body_fn = body_fn  # Actual python function

    def __call__(self, *args):
        if len(args) != len(self.param_names):
            # Added a more helpful error message showing expected vs received
            raise ValueError(f"Argument count mismatch: expected {len(self.param_names)}, got {len(args)}")
        
        # Map parameter name with its value
        arg_map = dict(zip(self.param_names, args))

        # Evaluate the body with these arguments
        # FIX: We use ** to unpack the dictionary into keyword arguments.
        # This allows you to define functions like 'def my_logic(a, b): ...' 
        # instead of 'def my_logic(args_dict): ...'
        result = self.body_fn(**arg_map)
        return result

# Unknowns propagate naturally through the function body.
# Intervals also propagate and combine automatically.
# Symbolics are preserved when any operand is symbolic or unknown.