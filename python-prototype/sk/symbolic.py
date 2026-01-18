from .value import SValue
from .kind import SKind
from .ops import product, add, divide, power

def simplify(expr, operands):
    """
    Simplifies trivial expressions for efficiency.
    Returns SValue if completely numeric, else keeps symbolic.
    """
    # If any operand is symbolic, keep symbolic
    if any(isinstance(op, SSymbolic) for op in operands):
        return SSymbolic(expr, operands)

    # Flatten resolved operands (they are now all SValue)
    operands = [op if not isinstance(op, SSymbolic) else op.resolve() for op in operands]

    # ADD
    if expr == "add":
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 0)]
        if not operands:
            return SValue(0)
        if len(operands) == 1:
            return operands[0]

    # SUB
    elif expr == "sub":
        if operands[1].kind == SKind.known and operands[1].lower == 0:
            return operands[0]
        if operands[0].structurally_equal(operands[1]):
            if operands[0].kind == SKind.interval:
                return SSymbolic(expr, operands) # Cannot simplify intervals
            return SValue(0)

    # MUL
    elif expr == "mul":
        for op in operands:
            if op.kind == SKind.known and op.lower == 0:
                return SValue(0)
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 1)]
        if not operands:
            return SValue(1)
        if len(operands) == 1:
            return operands[0]

    # DIV
    elif expr == "div":
        if operands[0].kind == SKind.known and operands[0].lower == 0:
            return SValue(0)
        if operands[1].kind == SKind.known and operands[1].lower == 1:
            return operands[0]
        if operands[0].structurally_equal(operands[1]):
            if operands[0].kind == SKind.interval:
                return SSymbolic(expr, operands)
            return SValue(1)

    # POW
    elif expr == "pow":
        if operands[1].kind == SKind.known and operands[1].lower == 0:
            return SValue(1)
        if operands[1].kind == SKind.known and operands[1].lower == 1:
            return operands[0]
        if operands[0].kind == SKind.known and operands[0].lower == 0:
            return SValue(0)
        if operands[0].kind == SKind.known and operands[0].lower == 1:
            return operands[0]

    # Otherwise, keep symbolic
    return SSymbolic(expr, operands)



class SSymbolic:
    def __init__(self, expr, operands):
        self.expr = expr
        # operands must be SValue objects
        self.operands = operands

        self._valid = False
        self._cached_value = None

        for op in operands:
            if isinstance(op, SValue):
                op.add_dependent(self)

    @property
    def kind(self):
        for op in self.operands:
            if op.kind in (SKind.unknown, SKind.symbolic):
                return SKind.symbolic
        return SKind.known

    def __repr__(self):
        def _repr(op):
            if isinstance(op, SValue) or isinstance(op, SSymbolic):
                return repr(op)
            return str(op)  # fallback for raw numbers
        return f"{self.expr}({', '.join(_repr(op) for op in self.operands)})"

    def resolve(self):
        # Resolve all operands recursively
        resolved = [op.resolve() if isinstance(op, SSymbolic) else op for op in self.operands]

        # If any operand is unknown or symbolic, keep symbolic
        for op in resolved:
            if op.kind in (SKind.unknown, SKind.symbolic):
                return simplify(self.expr, resolved)

        # Compute numeric result
        result = resolved[0]
        for o in resolved[1:]:
            if self.expr == "add":
                result = add(result, o)
            elif self.expr == "sub":
                result = add(result, -o)
            elif self.expr == "mul":
                result = product(result, o)
            elif self.expr == "div":
                result = divide(result, o)
            elif self.expr == "pow":
                result = power(result, o)
            else:
                raise ValueError(f"Unknown operation: {self.expr}")

        return result
    
    def invalidate(self):
        self._valid = False
        self._cached_value = None

    # == operator overloads ==
    def __add__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return add(self, other)
        return SSymbolic("add", [self, other])

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return add(self, -other)
        return SSymbolic("sub", [self, other])

    def __rsub__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return add(other, -self)
        return SSymbolic("sub", [other, self])

    def __mul__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return product(self, other)
        return SSymbolic("mul", [self, other])

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return divide(self, other)
        return SSymbolic("div", [self, other])

    def __rtruediv__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return divide(other, self)
        return SSymbolic("div", [other, self])

    def __pow__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return power(self, other)
        return SSymbolic("pow", [self, other])

    def __rpow__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind not in (SKind.symbolic, SKind.unknown) and other.kind not in (SKind.symbolic, SKind.unknown):
            return power(other, self)
        return SSymbolic("pow", [other, self])

class SQuietSymbolic(SSymbolic):
    """Like SSymbolic, but hides unresolved formulas."""
    def resolve(self):
        resolved = super().resolve()
        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()  # unknown
        return resolved

    def __repr__(self):
        return repr(self.resolve())


class SConstSymbolic(SSymbolic):
    """Immutable symbolic expression. Once created, operands cannot change."""
    def __init__(self, expr, operands):
        super().__init__(expr, tuple(operands))

    @property
    def operands(self):
        return self._operands

    @operands.setter
    def operands(self, value):
        if hasattr(self, "_operands"):
            raise AttributeError("ConstSymbolic operands cannot be modified")
        self._operands = tuple(value)


class SConstQuietSymbolic(SQuietSymbolic):
    """Immutable quiet symbolic expression."""
    def __init__(self, expr, operands):
        super().__init__(expr, tuple(operands))

    @property
    def operands(self):
        return self._operands

    @operands.setter
    def operands(self, value):
        if hasattr(self, "_operands"):
            raise AttributeError("ConstQuietSymbolic operands cannot be modified")
        self._operands = tuple(value)

    def resolve(self):
        resolved = super(SQuietSymbolic, self).resolve()  # call SSymbolic.resolve()
        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()
        return resolved