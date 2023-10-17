from gcounter import GCounter

# The PNCounter is a CRDT that can increment and decrement values from the counter. 
# To do this, it contains two instances of GCounter. 
class PNCounter:
    def __init__(self, id):
        self.id = id
        self.inc = GCounter(id)     # Incremental GCounter
        self.dec = GCounter(id)     # Decremental GCounter 

    def increment(self):
        self.inc.increment()

    def decrement(self):
        self.dec.increment()

    # The value is simply the subtraction between the inc and dec counters. 
    def value(self):
        return self.inc.value() - self.dec.value()

    def merge(self, other):
        self.inc.merge(other.inc)
        self.dec.merge(other.dec)