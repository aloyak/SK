from .value import SValue
from .kind import SKind

def certain(cond: SValue) -> SValue:
    """Returns Strue() if cond is certainly true, Sfalse() otherwise."""
    if cond.kind == SKind.known and cond.lower == 1:
        return SValue(1)  # Strue
    if cond.kind == SKind.interval and cond.lower == 1:
        return SValue(0, 1) # Same as a Spartial()
    return SValue(0)

def impossible(cond: SValue) -> SValue:
    if cond.kind == SKind.known:
        return SValue(1) if cond.lower == 0 else SValue(0)
    
    if cond.kind == SKind.interval:
        return SValue(0,1)  # Could be true partially
    
    # unknown or symbolic
    return SValue(0)

def possible(cond: SValue) -> SValue:
    """Returns Strue if cond could possibly be true (even partially), Sfalse otherwise."""
    if cond.kind == SKind.known and cond.lower == 0:
        return SValue(0)
    return SValue(1)

def known(cond: SValue) -> SValue:
    """Returns Strue if cond is fully known (0 or 1), Sfalse otherwise."""
    if cond.kind == SKind.known:
        return SValue(1)
    return SValue(0)
