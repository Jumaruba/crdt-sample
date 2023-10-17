import unittest
from ..gcounter import GCounter 

# Execute at python folder: 
# python3 -m state-based.tests.test_gcounter
class TestGCounter(unittest.TestCase):
    def test_add(self):
        gcounter = GCounter("banana") 
        gcounter.increment() 
        gcounter.increment() 
        self.assertEqual(gcounter.value(), 2) 

    def test_merge(self):
        gcounter1 = GCounter("banana") 
        gcounter1.increment() 
        gcounter1.increment() 

        gcounter2 = GCounter("babana1")
        for _ in range(3):
            gcounter2.increment() 

        self.assertEqual(gcounter1.value(), 2)
        self.assertEqual(gcounter2.value(), 3)
        gcounter1.merge(gcounter2)
        self.assertEqual(gcounter1.value(), 5)
        gcounter2.merge(gcounter1) 
        self.assertEqual(gcounter2.value(), 5)

 

if __name__ == '__main__': 
    unittest.main() 
