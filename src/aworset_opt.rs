use core::hash::Hash;
use std::{collections::{HashSet}, fmt::Display};
use std::fmt::Debug;
use crate::{NodeId, DotContext};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AworsetOpt<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub set: HashSet<(NodeId, E, i64)>,   
    pub cc: DotContext<NodeId> // Equivalent to the cc in aworset.rs
}

impl<E> AworsetOpt<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: NodeId) -> Self {
        Self {
            id, 
            set: HashSet::new(),
            cc: DotContext::new()
        }
    }
    
    pub fn elements(&self) -> HashSet<E>{
        let mut res: HashSet<E> = HashSet::new();
        for (_, e, _) in self.set.iter(){
            res.insert(e.clone());
        }
        res
    }

    pub fn add(&mut self, element: E) {
        let dot = self.cc.makedot(&self.id);
        self.set.insert((dot.0, element, dot.1));
    }

    pub fn rm(&mut self, element: E) {
        self.set= self.set.drain().filter(|(_, e, _) | {
            if element == *e { return false; }   
            true
        }).collect();
    }

    pub fn join(&mut self, other: &Self){
        // Intersentions and elements not known by other.
        self.set = self.set.drain().filter(|v|
            other.set.contains(v) || !other.cc.dotin(&(v.0.clone(), v.2)))
        .collect();
        
        // Elements known by other, but not by self.
        for entry in other.set.iter() {
            if !self.cc.dotin(&(entry.0.clone(), entry.2)) {
                self.set.insert(entry.clone());
            }
        }

        self.cc.join(&other.cc);
    }
}

