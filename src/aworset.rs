use core::hash::Hash;
use std::{collections::{HashMap, HashSet}, fmt::Display};
use std::fmt::Debug;
use crate::NodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Aworset<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub cc: HashSet<(NodeId, u64)>,
    pub set: HashSet<(NodeId, E, u64)>,
    pub local_counter: u64,
}

impl<E> Aworset<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            cc: HashSet::new(),
            set: HashSet::new(),
            local_counter: 1,
        }
    }

    pub fn add(&mut self, element: E) {
        self.cc.insert((self.id.clone(), self.local_counter));
        self.set
            .insert((self.id.clone(), element, self.local_counter));

        self.local_counter += 1;
    }

    pub fn rm(&mut self, element: &E) {
        self.set = self.set.drain().filter(|(_, ele, _)| ele != element).collect();
    }

    pub fn join(&mut self, other: &Self){
        self.set = self.set
            .drain()
            .filter(|v| 
                other.set.contains(v) || !other.cc.contains(&(v.0.clone(),v.2)))
            .collect();
        
        for entry in other.set.iter() {
            if !self.cc.contains(&(entry.0.clone(), entry.2)) {
                self.set.insert(entry.clone());
            }
        }

        self.cc.extend(other.cc.clone());
    }
}
