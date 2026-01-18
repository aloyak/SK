from .kind import SKind

class SValue:
    def __init__(self, lower=None, higher=None):
        self.lower = lower
        self.higher = higher

        defined_count = sum(x is not None for x in (lower, higher))

        if defined_count == 0:
            self.kind = SKind.unknown
        elif defined_count == 1:
            self.kind = SKind.known
            # Ensure lower holds the known value
            if self.lower is None:
                self.lower = self.higher
            self.higher = None
        elif defined_count == 2 and lower == higher:
            self.kind = SKind.known
            # Ensure lower holds the value
            self.lower = lower
            self.higher = None
        else:
            self.kind = SKind.interval
            if lower > higher:
                raise ValueError(f"Invalid Interval: lower ({lower}) cannot be greater than higher ({higher})")

    def resolve(self):
        return self

    def isKnownZero(self) -> bool:
        return self.kind is SKind.known and (self.lower == 0 or self.higher == 0)

    def setKnown(self, value):
        self.lower = value
        self.higher = None
        self.kind = SKind.known

    def setUnknown(self):
        self.lower = None
        self.higher = None
        self.kind = SKind.unknown

    def setInterval(self, low, high):
        if low > high:
            raise ValueError("Invalid interval")
        self.lower = low
        self.higher = high
        self.kind = SKind.interval

    def __repr__(self):
        """Returns a string representation for easier debugging."""

        if self.kind is SKind.unknown:
            return "unknown"
        if self.kind is SKind.known:
            val = self.lower if self.lower is not None else self.higher
            return f"{val}"
        return f"[{self.lower}..{self.higher}]"

    def __neg__(self):
        # Unknown stays unknown
        if self.kind is SKind.unknown:
            return SValue()

        # Known: negate the single value
        if self.kind is SKind.known:
            val = self.lower if self.lower is not None else self.higher
            return SValue(-val)

        # Interval: both bounds exist here
        return SValue(-self.higher, -self.lower)
    
    # == operator overloads ==
    def __add__(self, other):
        from .ops import add
        if not isinstance(other, SValue):
            other = SValue(other)

        # Create symbolic if either operand is unknown or symbolic
        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("add", [self, other])

        # Otherwise compute numeric
        return add(self, other)

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        from .ops import add
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("sub", [self, other])

        return add(self, -other)

    def __rsub__(self, other):
        from .ops import add
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("sub", [other, self])

        return add(other, -self)

    def __mul__(self, other):
        from .ops import product
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("mul", [self, other])

        return product(self, other)

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        from .ops import divide
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("div", [self, other])

        return divide(self, other)

    def __rtruediv__(self, other):
        from .ops import divide
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("div", [other, self])

        return divide(other, self)

    def __pow__(self, other):
        from .ops import power
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("pow", [self, other])

        return power(self, other)

    def __rpow__(self, other):
        from .ops import power
        if not isinstance(other, SValue):
            other = SValue(other)

        if self.kind in (SKind.unknown, SKind.symbolic) or other.kind in (SKind.unknown, SKind.symbolic):
            from .symbolic import SSymbolic
            return SSymbolic("pow", [other, self])

        return power(other, self)

    def bounds(self):
        """
        Returns a tuple (min, max) for this SValue.
        - Known: both values are the same
        - Interval: lower and higher
        """
        if self.kind is SKind.known:
            val = self.lower if self.lower is not None else self.higher
            return val, val
        elif self.kind is SKind.interval:
            return self.lower, self.higher
        else:
            raise ValueError("Cannot get bounds of Unknown SValue")

    def __eq__(self, other):
        if not isinstance(other, SValue):
            return False
        if self.kind != other.kind:
            return False
        # Compare lower and higher with float/int tolerance
        return self.lower == other.lower and self.higher == other.higher

#   sk:

#   basic values
#   x = [3..5]    => interval from 3 to 5
#   y = 2         => y is exacly 2
#   z = unknown   => z is a value currently not known