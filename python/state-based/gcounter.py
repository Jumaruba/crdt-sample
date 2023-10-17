# The GCounter is a Only-Grow counter. A counter that only increments. 
class GCounter: 
    def __init__(self, id): 
        self.id = id 
        self.counters = {} 

    # Increment the position in the Counter 
    def increment(self): 
        self.counters[self.id] = self.counters.get(self.id, 0) + 1 

    # The value is the sum of all entries in the counters. 
    def value(self): 
        return sum(self.counters.values()) 

    
    def merge(self, other): 
        for id, counter in other.counters.items(): 
            self.counters[id] = max(counter, self.counters.get(id, 0))
