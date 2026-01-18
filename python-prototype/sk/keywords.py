from .value import SValue
from .symbolic import SSymbolic, SQuietSymbolic, SConstSymbolic, SConstQuietSymbolic

def Sunknown():
    return SValue()

def Sknown(val):
    return SValue(val)

def Sinterval(lower, higher):
    return SValue(lower, higher)


def Ssymbolic(expr, operands):
    return SSymbolic(expr, operands)

def Squiet(expr, operands):
    return SQuietSymbolic(expr, operands)


def Const(obj): # supports only constant symbolic and constant quiet symbolic
    if isinstance(obj, SSymbolic) and not isinstance(obj, SQuietSymbolic):
        return SConstSymbolic(obj.expr, obj.operands)
    elif isinstance(obj, SQuietSymbolic):
        return SConstQuietSymbolic(obj.expr, obj.operands)
    else:
        raise TypeError(f"Const() not implemented for type {type(obj)}")