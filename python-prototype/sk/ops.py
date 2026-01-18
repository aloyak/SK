from .value import SValue
from .kind import SKind

def add(a: SValue, b: SValue) -> SValue:
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()  # unknown dominates

    if a.kind is SKind.known and b.kind is SKind.known:
        return SValue(a.lower + b.lower)

    # Intervals
    low = (a.lower if a.lower is not None else a.higher) + (b.lower if b.lower is not None else b.higher)
    high = (a.higher if a.higher is not None else a.lower) + (b.higher if b.higher is not None else b.lower)
    return SValue(min(low, high), max(low, high))


def product(a: SValue, b: SValue) -> SValue:
    if a.isKnownZero() or b.isKnownZero():
        return SValue(0)  # 0 * unknown = 0

    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()  # unknown dominates

    if a.kind is SKind.known and b.kind is SKind.known:
        return SValue(a.lower * b.lower)

    # Interval multiplication
    a_min = a.lower if a.lower is not None else a.higher
    a_max = a.higher if a.higher is not None else a.lower
    b_min = b.lower if b.lower is not None else b.higher
    b_max = b.higher if b.higher is not None else b.lower

    products = [
        a_min * b_min,
        a_min * b_max,
        a_max * b_min,
        a_max * b_max,
    ]
    return SValue(min(products), max(products))


def divide(a: SValue, b: SValue) -> SValue:
    # Unknown dominates
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()  

    # Check zero in denominator
    if b.kind is SKind.known:
        if b.lower == 0:
            raise ZeroDivisionError("division by zero")
        return SValue(a.lower / b.lower) if a.kind is SKind.known else SValue(*(v / b.lower for v in a.bounds()))
    
    if b.kind is SKind.interval:
        if b.lower <= 0 <= b.higher:
            raise ZeroDivisionError("division by interval containing zero")
        # safe division: known numerator / interval denominator
        if a.kind is SKind.known:
            vals = [a.lower / b.lower, a.lower / b.higher]
            return SValue(min(vals), max(vals))
        # interval numerator / interval denominator
        if a.kind is SKind.interval:
            a_lo, a_hi = a.bounds()
            b_lo, b_hi = b.bounds()
            candidates = [
                a_lo / b_lo, a_lo / b_hi,
                a_hi / b_lo, a_hi / b_hi
            ]
            return SValue(min(candidates), max(candidates))

    raise ZeroDivisionError(f"division of {a.kind} by {b.kind} is not allowed")


def power(a: SValue, b: SValue) -> SValue:
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()

    # Both known
    if a.kind is SKind.known and b.kind is SKind.known:
        # 0**0 = 1
        if a.lower == 0 and b.lower == 0:
            return SValue(1)
        if a.lower == 0 and b.lower < 0:
            raise ZeroDivisionError("0 cannot be raised to a negative power")
        return SValue(a.lower ** b.lower)

    # Interval ** Known
    if a.kind is SKind.interval and b.kind is SKind.known:
        e = b.lower
        lo, hi = a.bounds()

        # Check 0**negative in interval
        if lo <= 0 <= hi and e < 0:
            raise ZeroDivisionError("Interval includes zero, cannot raise to negative exponent")

        # Negative base with fractional exponent not allowed
        if lo < 0 and not float(e).is_integer():
            raise ValueError("Cannot raise negative interval to fractional exponent")

        vals = [lo ** e, hi ** e]
        return SValue(min(vals), max(vals))

    # Known ** Interval
    if a.kind is SKind.known and b.kind is SKind.interval:
        lo, hi = b.bounds()
        if a.lower < 0 and lo != int(lo):
            raise ValueError("Negative base with non-integer exponent interval not allowed")
        vals = [a.lower ** lo, a.lower ** hi]
        return SValue(min(vals), max(vals))

    # Interval ** Interval
    if a.kind is SKind.interval and b.kind is SKind.interval:
        raise ValueError("Exponentiation with interval exponent not supported")

    raise ValueError("Unsupported exponentiation combination")
