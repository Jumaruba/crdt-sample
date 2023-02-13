use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

use crate::NodeId;

pub struct GCounter<V>
    where V: Display + AddAssign + Clone + Copy + for<'v> Sum<&'v V> + Ord + Add<Output=V> + Default,
{
    /// The identification number of the delta.
    pub id: NodeId,
    // The current state of the counter.
    pub counter: HashMap<NodeId, V>,
    /// The default value of V. For example, the default value of i32 is 0.
    pub default_value: V,
}

impl<V> GCounter<V>
    where V: Display + AddAssign + Clone + Copy + for<'v> Sum<&'v V> + Ord + Add<Output = V> + Default,
{
    pub fn new(id: NodeId) -> Self {
        GCounter {
            id,
            counter: HashMap::new(),
            default_value: Default::default(),
        }
    }
    /// Returns the current value of the local counter.
    pub fn local(&self) -> &V {
        self.counter.get(&self.id).unwrap_or(&self.default_value)
    }

    /// Returns the sum of all replica values in the counter.
    pub fn read(&self) -> V {
        self.counter.values().sum()
    }

    /// A mutator that performs an increment of `to_sum` in the counter.
    ///
    /// # Example
    /// ```
    /// use crdt_sample::{GCounter,NodeId};
    /// 
    /// let id = NodeId::new(1, "a".to_string());
    /// let mut gcounter: GCounter<i32> = GCounter::new(id);
    /// 
    /// gcounter.inc(Some(1));
    /// let result_local = gcounter.local();    // Reads the local replica
    /// let result_read = gcounter.read();      // Sum of all counters.
    /// assert_eq!(*result_local, 1);
    /// assert_eq!(result_read, 1);
    /// ```
    pub fn inc(&mut self, to_sum: Option<V>){
        let inc_value: V = to_sum.unwrap();
        self.counter
            .entry(self.id.clone())
            .and_modify(|val| *val = max(inc_value.clone() + *val, *val))
            .or_insert(inc_value);
    }

    /// Joins the delta_gcounter with a delta or another delta_gcounter.
    ///
    /// # Examples
    /// ```
    /// use crdt_sample::{GCounter, NodeId};
    /// let id1 = NodeId::new(1, "a".to_string());
    /// let id2 = NodeId::new(2, "a".to_string());
    /// let id3 = NodeId::new(3, "a".to_string());
    /// let mut gcounter_1: GCounter<i32> = GCounter::new(id1);
    /// let mut gcounter_2: GCounter<i32> = GCounter::new(id2);
    /// let mut gcounter_3: GCounter<i32> = GCounter::new(id3);
    ///
    /// gcounter_1.inc(Some(3));
    /// gcounter_2.inc(Some(5));
    /// gcounter_3.inc(Some(1));
    ///
    /// // Join gcounter_1
    /// gcounter_3.join(&gcounter_1);
    /// let result_read = gcounter_3.read();
    /// let result_local = gcounter_3.local();
    /// assert_eq!(result_read, 4);
    /// assert_eq!(*result_local, 1);
    ///
    /// // join gcounter_2
    /// gcounter_3.join(&gcounter_2);
    /// let result_read = gcounter_3.read();
    /// let result_local = gcounter_3.local();
    /// assert_eq!(result_read, 9);
    /// assert_eq!(*result_local, 1);
    /// ```
    pub fn join(&mut self, gcounter: &GCounter<V>) {
        for (key, value) in gcounter.counter.iter() {
            let gcounter_value = gcounter.counter.get(key).unwrap();
            let new_value = max(gcounter_value, value);
            self.counter.insert(key.clone(), *new_value);
        }
    }
}

