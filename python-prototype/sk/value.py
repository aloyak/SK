from .kind import SKind

class SValue:
    def __init__(self, lower=None, higher=None):
        self.lower = lower
        self.higher = higher
        self._dependents = set() 

        defined_count = sum(x is not None for x in (lower, higher))

        if defined_count == 0:
            self.kind = SKind.unknown
        elif defined_count == 1:
            self.kind = SKind.known
            if self.lower is None:
                self.lower = self.higher
            self.higher = None
        elif defined_count == 2 and lower == higher:
            self.kind = SKind.known
            self.lower = lower
            self.higher = None
        else:
            self.kind = SKind.interval
            # Only validate range if both are numeric
            if isinstance(lower, (int, float)) and isinstance(higher, (int, float)):
                if lower > higher:
                    raise ValueError(f"Invalid Interval: lower ({lower}) cannot be greater than higher ({higher})")

    def resolve(self):
        return self

    def isKnownZero(self) -> bool:
        return self.kind is SKind.known and self.lower == 0

    def setKnown(self, value):
        self.lower = value
        self.higher = None
        self.kind = SKind.known
        self._notify_dependents()

    def setUnknown(self):
        self.lower = None
        self.higher = None
        self.kind = SKind.unknown
        self._notify_dependents()

    def setInterval(self, low, high):
        self.lower = low
        self.higher = high
        self.kind = SKind.interval
        self._notify_dependents()

    def __repr__(self):
        if self.kind is SKind.unknown:
            return "unknown"
        if self.kind is SKind.known:
            return str(self.lower)
        if self.kind is SKind.interval:
            return f"[{self.lower}..{self.higher}]"
        if self.kind is SKind.symbolic:
            return f"symbolic({self.expr})"

    def __neg__(self):
        if self.kind is SKind.unknown:
            return SValue()
        if self.kind is SKind.known:
            return SValue(-self.lower)
        return SValue(-self.higher, -self.lower)
    
    def add_dependent(self, symbolic):
        self._dependents.add(symbolic)

    def remove_dependent(self, symbolic):
        self._dependents.discard(symbolic)

    def _notify_dependents(self):
        for sym in list(self._dependents):
            sym.invalidate()

    def __add__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("add", [self, other])

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("sub", [self, other])

    def __rsub__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("sub", [other, self]).resolve()

    def __mul__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("mul", [self, other]).resolve()

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("div", [self, other]).resolve()

    def __rtruediv__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("div", [other, self]).resolve()

    def __pow__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("pow", [self, other]).resolve()

    def __rpow__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        return SQuietSymbolic("pow", [other, self]).resolve()

    def bounds(self):
        if self.kind is SKind.known:
            return self.lower, self.lower
        elif self.kind is SKind.interval:
            return self.lower, self.higher
        else:
            raise ValueError("Cannot get bounds of Unknown SValue")

    def __eq__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("eq", [self, other])
        from .ops_boolean import equal
        return equal(self, other)

    def __gt__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("gt", [self, other])
        from .ops_boolean import greater_than
        return greater_than(self, other)
    
    def __lt__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("lt", [self, other])
        from .ops_boolean import less_than
        return less_than(self, other)
    
    def __ge__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("ge", [self, other])
        from .ops_boolean import greater_equal
        return greater_equal(self, other)
    
    def __le__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("le", [self, other])
        from .ops_boolean import less_equal
        return less_equal(self, other)

    def __ne__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind in (SKind.symbolic, SKind.unknown) or other.kind in (SKind.symbolic, SKind.unknown):
            return SQuietSymbolic("ne", [self, other])
        from .ops_boolean import not_equal
        return not_equal(self, other)
    
    def structurally_equal(self, other):
        return (
            isinstance(other, SValue)
            and self.kind == other.kind
            and self.lower == other.lower
            and self.higher == other.higher
        )

    def __bool__(self):
        raise TypeError(
            "Cannot convert uncertain boolean to Python bool. "
            "Use epistemic if instead."
        )
    
    def __and__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue): other = SValue(other)
        return SQuietSymbolic("and", [self, other])

    def __or__(self, other):
        from .symbolic import SQuietSymbolic
        if not isinstance(other, SValue): other = SValue(other)
        return SQuietSymbolic("or", [self, other])

    def __invert__(self):
        from .symbolic import SQuietSymbolic
        return SQuietSymbolic("not", [self])