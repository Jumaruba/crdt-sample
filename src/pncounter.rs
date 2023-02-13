use std::{
    fmt::Display,
    iter::Sum,
    ops::AddAssign,
    ops::{Add, Sub},
};

use crate::{GCounter, NodeId};

pub struct PnCounter<V>
where
    V: Display
        + AddAssign
        + Clone
        + Copy
        + for<'v> Sum<&'v V>
        + Ord
        + Add<Output = V>
        + Sub<Output = V>
        + Default,
{
    pub id: NodeId,
    pub pos_counter: GCounter<V>,
    pub neg_counter: GCounter<V>,
    pub default_value: V,
}

impl<V> PnCounter<V>
where
    V: Display
        + AddAssign
        + Clone
        + Copy
        + for<'v> Sum<&'v V>
        + Ord
        + Add<Output = V>
        + Sub<Output = V>
        + Default,
{
    pub fn new(id: NodeId) -> Self {
        PnCounter {
            id: id.clone(),
            pos_counter: GCounter::new(id.clone()),
            neg_counter: GCounter::new(id.clone()),
            default_value: Default::default(),
        }
    }

    pub fn inc(&mut self, to_sum: Option<V>) {
        self.pos_counter.inc(to_sum);
    }

    pub fn dec(&mut self, to_sub: Option<V>) {
        self.neg_counter.inc(to_sub);
    }

    pub fn local(&self) -> V {
        self.pos_counter.local().clone() - self.neg_counter.local().clone()
    }

    pub fn read(&self) -> V {
        self.pos_counter.read() - self.neg_counter.read()
    }

    pub fn join(&mut self, pncounter: &PnCounter<V>) {
        self.pos_counter.join(&pncounter.pos_counter);
        self.neg_counter.join(&pncounter.neg_counter);
    }
}
