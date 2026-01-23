from .value import SValue
from .kind import SKind
from .ops import product, add, divide, power

def simplify(expr, operands):
    if any(isinstance(op, SSymbolic) for op in operands):
        return SSymbolic(expr, operands)

    operands = [op if not isinstance(op, SSymbolic) else op.resolve() for op in operands]

    if expr == "and":
        if any(op.kind == SKind.known and op.lower == 0 for op in operands):
            return SValue(0)
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 1)]
        if not operands: return SValue(1)
        if len(operands) == 1: return operands[0]

    elif expr == "or":
        if any(op.kind == SKind.known and op.lower == 1 for op in operands):
            return SValue(1)
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 0)]
        if not operands: return SValue(0)
        if len(operands) == 1: return operands[0]

    elif expr == "not":
        if operands[0].kind == SKind.known:
            return SValue(0) if operands[0].lower == 1 else SValue(1)
        if operands[0].kind == SKind.interval:
            return SValue(0, 1)

    if expr == "add":
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 0)]
        if not operands:
            return SValue(0)
        if len(operands) == 1:
            return operands[0]

    elif expr == "sub":
        if operands[1].kind == SKind.known and operands[1].lower == 0:
            return operands[0]
        # Identity: x - x is always 0, even for intervals
        if operands[0] is operands[1] or operands[0].structurally_equal(operands[1]):
            return SValue(0)

    elif expr == "mul":
        for op in operands:
            if op.kind == SKind.known and op.lower == 0:
                return SValue(0)
        operands = [op for op in operands if not (op.kind == SKind.known and op.lower == 1)]
        if not operands:
            return SValue(1)
        if len(operands) == 1:
            return operands[0]

    return SSymbolic(expr, operands)

class SSymbolic:
    def __init__(self, expr, operands):
        self.expr = expr
        self.operands = operands
        self._valid = False
        self._cached_value = None
        for op in operands:
            if isinstance(op, SValue):
                op.add_dependent(self)

    @property
    def kind(self):
        return SKind.symbolic

    def resolve(self):
        if self._valid:
            return self._cached_value

        resolved = [op.resolve() if hasattr(op, "resolve") else op for op in self.operands]
        
        simplified = simplify(self.expr, resolved)
        if not isinstance(simplified, SSymbolic):
            self._cached_value = simplified
            self._valid = True
            return simplified

        if self.expr == "not":
            from .ops_boolean import logic_not
            result = logic_not(resolved[0])
        elif self.expr == "and":
            from .ops_boolean import logic_and
            result = resolved[0]
            for o in resolved[1:]:
                result = logic_and(result, o)
        elif self.expr == "or":
            from .ops_boolean import logic_or
            result = resolved[0]
            for o in resolved[1:]:
                result = logic_or(result, o)
        elif self.expr == "eq":
            from .ops_boolean import equal
            result = equal(resolved[0], resolved[1])
        elif self.expr == "add":
            from .ops import add as op_add
            result = resolved[0]
            for o in resolved[1:]:
                result = op_add(result, o)
        elif self.expr == "sub":
            from .ops import add as op_add
            result = op_add(resolved[0], -resolved[1])
        elif self.expr == "mul":
            result = resolved[0]
            for o in resolved[1:]:
                result = product(result, o)
        elif self.expr == "div":
            result = divide(resolved[0], resolved[1])
        elif self.expr == "pow":
            result = power(resolved[0], resolved[1])
        else:
            result = self

        self._cached_value = result
        self._valid = True
        return result

    def invalidate(self):
        self._valid = False
        self._cached_value = None

    def __repr__(self):
        ops_map = {"add": "+", "sub": "-", "mul": "*", "div": "/", "pow": "**"}
        op_char = ops_map.get(self.expr, self.expr)
        return f"({f' {op_char} '.join(map(repr, self.operands))})"

    def __add__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("add", [self, other])

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("sub", [self, other])

    def __rsub__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("sub", [other, self])

    def __mul__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("mul", [self, other])

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("div", [self, other])

    def __rtruediv__(self, other):
        if not isinstance(other, (SValue, SSymbolic)):
            other = SValue(other)
        return SSymbolic("div", [other, self])
    
    def __and__(self, other):
        if not isinstance(other, (SValue, SSymbolic)): other = SValue(other)
        return SQuietSymbolic("and", [self, other])

    def __or__(self, other):
        if not isinstance(other, (SValue, SSymbolic)): other = SValue(other)
        return SQuietSymbolic("or", [self, other])

    def __invert__(self): # ~x 
        return SQuietSymbolic("not", [self])

class SQuietSymbolic(SSymbolic):
    def resolve(self):
        resolved = super().resolve()
        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()
        return resolved

    def __repr__(self):
        return repr(self.resolve())

class SConstSymbolic(SSymbolic):
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
        resolved = super(SQuietSymbolic, self).resolve()
        if isinstance(resolved, SSymbolic) and resolved.kind == SKind.symbolic:
            return SValue()
        return resolved