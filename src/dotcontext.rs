use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

/// Tries to optimize mapping.
/// Source: https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DotContext<K: PartialEq + Eq + Hash + Clone + Debug> {
    pub cc: HashMap<K, i64>,   // Compact Causal Context
    pub dc: HashSet<(K, i64)>, // Dot Cloud
}

impl<K: PartialEq + Eq + Hash + Clone + Debug> DotContext<K> {
    pub fn new() -> Self {
        Self {
            cc: HashMap::new(),
            dc: HashSet::new(),
        }
    }

    /// Verifies if the received argument was already seen.
    pub fn dotin(&self, d: &(K, i64)) -> bool {
        if let Some(&v) = self.cc.get(&d.0) {
            if d.1 <= v {
                return true;
            }
        } else if let Some(_) = self.dc.get(d) {
            return true;
        }
        return false;
    }

    /// Creates a new dot considering that the dots are already compact.
    pub fn makedot(&mut self, id: &K) -> (K, i64) {
        match self.cc.get_mut(id) {
            // No entry, then create one.
            None => {self.cc.insert(id.clone(), 1);},
            // There is an entry, then update it.
            Some(v) => {*v += 1;}
        }
        return (id.clone(), self.cc.get(id).unwrap().clone());
    }

    /// Adds a dot to the struct.
    pub fn insert_dot(&mut self, dot: &(K, i64), compact: Option<bool>) {
        self.dc.insert(dot.clone());
        match compact {
            Some(true) => self.compact(),
            Some(false) => return,
            None => self.compact(),
        }
    }

    pub fn join(&mut self, other: &Self) {
        for (other_k, &other_val) in other.cc.iter() {
            
            match self.cc.get_mut(&other_k) {
                // No previous record, then insert. 
                None => {
                    self.cc.insert(other_k.clone(), other_val);
                }, 
                // Get maximum between both.
                Some(self_val) => {
                    *self_val = max(*self_val, other_val);
                }
            }
        }

        self.union_dc(&other.dc);
        self.compact();
    }

    /// Performs the union between the self.dc and the one received. 
    fn union_dc(&mut self, dc: &HashSet<(K, i64)>) {
        for (id, val) in dc.iter() {
            self.dc.insert((id.clone(), val.clone()));
        }
    }

    pub fn compact(&mut self) {
        let mut repeat: bool;
        loop {
            repeat = false;
            self.dc = self
                .dc
                .drain()
                .filter(|(id, dc_count)| {
                    match self.cc.get_mut(id) {
                        // No CC entry.
                        None => {
                            // If starts, with 1 (not decoupled), can compact.
                            if *dc_count == 1 { 
                                self.cc.insert(id.clone(), *dc_count);
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                        }
                        // There is already a CC entry.
                        Some(cc_count) => {
                            // Contiguos, compact.
                            if *cc_count == *dc_count - 1 {
                                *cc_count += 1;
                                repeat = true;
                                return false; // Do not re-add it to dc.
                            }
                            // Has dc_count is already considered in cc.
                            else if *cc_count >= *dc_count {
                                return false; // Do not re-add it to dc.
                                              // No extra compact opportunities. Flag untoched.
                            }
                        }
                    }
                    return true; // cc_count <= dc_count.
                })
                .collect();

            if !repeat {
                break;
            }
        }
    }

}
