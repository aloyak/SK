from .kind import SKind

def epistemic_if(condition, if_fn, else_fn):
    resolved = condition.resolve() if hasattr(condition, "resolve") else condition

    if resolved.kind == SKind.known:
        if resolved.lower == 1:   # Strue
            return if_fn()
        else:                     # Sfalse
            return else_fn()
    else:
        # Partial or unknown: do nothing
        return None
