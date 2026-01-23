from .value import SValue
from .kind import SKind
from .keywords import *

def greater_than(a: SValue, b: SValue) -> SValue:
    """Return Strue if a > b, Sfalse if definitely false, Spartial if partially true."""
    from .symbolic import SSymbolic

    # Handle symbolic or unknown operands first
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("gt", [a, b])

    # Numeric / interval comparison
    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min > b_max:
        return Strue()
    if a_max <= b_min:
        return Sfalse()

    return Spartial()


def less_than(a: SValue, b: SValue) -> SValue:
    from .symbolic import SSymbolic

    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("lt", [a, b])

    # Use greater_than by flipping operands
    return greater_than(b, a)


def greater_equal(a: SValue, b: SValue) -> SValue:
    from .symbolic import SSymbolic

    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("ge", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min >= b_max:
        return Strue()
    if a_max < b_min:
        return Sfalse()

    return Spartial()


def less_equal(a: SValue, b: SValue) -> SValue:
    from .symbolic import SSymbolic

    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("le", [a, b])

    # Flip operands for simplicity
    return greater_equal(b, a)


def equal(a: SValue, b: SValue) -> SValue:
    from .symbolic import SSymbolic

    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("eq", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    # No overlap → definitely false
    if a_max < b_min or b_max < a_min:
        return Sfalse()

    # Both known and equal
    if a.kind == SKind.known and b.kind == SKind.known and a_min == b_min:
        return Strue()

    return Spartial()


def not_equal(a: SValue, b: SValue) -> SValue:
    from .symbolic import SSymbolic

    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        return SSymbolic("ne", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    # No overlap → definitely true
    if a_max < b_min or b_max < a_min:
        return Strue()

    # Both known and equal
    if a.kind == SKind.known and b.kind == SKind.known and a_min == b_min:
        return Sfalse()

    return Spartial()

def logic_and(a, b):
    if a.kind == SKind.known and b.kind == SKind.known:
        return SValue(1) if (a.lower == 1 and b.lower == 1) else SValue(0)
    if a.kind == SKind.unknown or b.kind == SKind.unknown:
        return SValue()
    return SValue(0, 1)

def logic_or(a, b):
    if a.kind == SKind.known and b.kind == SKind.known:
        return SValue(1) if (a.lower == 1 or b.lower == 1) else SValue(0)
    if a.kind == SKind.unknown or b.kind == SKind.unknown:
        return SValue()
    return SValue(0, 1)

def equal(a, b):
    if a.kind == SKind.known and b.kind == SKind.known:
        return SValue(1) if a.lower == b.lower else SValue(0)
    return SValue(0, 1)

def logic_not(val):
    if val.kind == SKind.known:
        return SValue(0) if val.lower == 1 else SValue(1)
    if val.kind == SKind.interval:
        return SValue(0, 1)
    return SValue()

def equal(a, b):
    if a.kind == SKind.known and b.kind == SKind.known:
        return SValue(1) if a.lower == b.lower else SValue(0)
    return SValue(0, 1)