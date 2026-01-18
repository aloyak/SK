from enum import Enum, auto

class SKind(Enum):
    unknown = auto() 
    known = auto() 
    interval = auto()
    symbolic = auto()
    
    #undefined = auto()