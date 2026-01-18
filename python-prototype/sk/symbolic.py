from .value import SValue
from .kind import SKind
from .ops import product, add, divide, power

class SSymbolic:
    def __init__(self, expr, operands):
        self.expr = expr
        # operands must be SValue objects
        self.operands = operands

    @property
    def kind(self):
        # If any operand is unknown/symbolic → symbolic
        for op in self.operands:
            if op.kind in (SKind.unknown, SKind.symbolic):
                return SKind.symbolic
        return SKind.known  # all numeric

    def __repr__(self):
        return f"{self.expr}({', '.join(map(str, self.operands))})"

    def resolve(self):
        resolved = []
        for op in self.operands:
            if isinstance(op, SSymbolic):
                op_val = op.resolve()
            else:
                op_val = op
            resolved.append(op_val)

        # Cannot resolve if any operand is unknown or symbolic
        for op in resolved:
            if op.kind in (SKind.unknown, SKind.symbolic):
                return SSymbolic(self.expr, resolved)

        # Compute numeric result
        result = resolved[0]
        if self.expr == "add":
            for o in resolved[1:]:
                result = add(result, o)
        elif self.expr == "sub":
            for o in resolved[1:]:
                result = add(result, -o)
        elif self.expr == "mul":
            for o in resolved[1:]:
                result = product(result, o)
        elif self.expr == "div":
            for o in resolved[1:]:
                result = divide(result, o)
        elif self.expr == "pow":
            for o in resolved[1:]:
                result = power(result, o)
        else:
            raise ValueError(f"Unknown operation: {self.expr}")

        return result
    
    # == operator overloads ==

    def __add__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return add(self, other)
        return SSymbolic("add", [self, other])

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return add(self, -other)
        return SSymbolic("sub", [self, other])

    def __rsub__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return add(other, -self)
        return SSymbolic("sub", [other, self])

    def __mul__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return product(self, other)
        return SSymbolic("mul", [self, other])

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return divide(self, other)
        return SSymbolic("div", [self, other])

    def __rtruediv__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return divide(other, self)
        return SSymbolic("div", [other, self])

    def __pow__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return power(self, other)
        return SSymbolic("pow", [self, other])

    def __rpow__(self, other):
        if not isinstance(other, SValue):
            other = SValue(other)
        if self.kind != SKind.symbolic and other.kind != SKind.symbolic:
            return power(other, self)
        return SSymbolic("pow", [other, self])

class SQuietSymbolic(SSymbolic):
    """Like SSymbolic, but hides unresolved formulas."""
    def resolve(self):
        # Call the normal SSymbolic resolve
        resolved = super().resolve()

        # If still unresolved symbolic → return unknown SValue
        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()  # unknown

        # Fully numeric → return resolved SValue
        return resolved
    
    def __repr__(self):
        return repr(self.resolve())

# Constant Symbolics
class SConstSymbolic(SSymbolic): # Immutable symbolic expression. Once created, operands cannot change
    def __init__(self, expr, operands):
        # Force operands to be a tuple to prevent mutation
        super().__init__(expr, tuple(operands))

    # Override any method that would mutate operands
    @property
    def operands(self):
        return self._operands

    @operands.setter
    def operands(self, value):
        # Prevent reassigning operands
        if hasattr(self, "_operands"):
            raise AttributeError("ConstSymbolic operands cannot be modified")
        self._operands = tuple(value)

class SConstQuietSymbolic(SQuietSymbolic):
    def __init__(self, expr, operands):
        # force operands to tuple to prevent mutation
        super().__init__(expr, tuple(operands))

    # Prevent reassigning operands
    @property
    def operands(self):
        return self._operands

    @operands.setter
    def operands(self, value):
        if hasattr(self, "_operands"):
            raise AttributeError("ConstQuietSymbolic operands cannot be modified")
        self._operands = tuple(value)

    def resolve(self):
        # Use the quiet resolve logic
        resolved = super(SQuietSymbolic, self).resolve()  # call SSymbolic.resolve() directly

        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()  # unresolved → unknown

        return resolved
