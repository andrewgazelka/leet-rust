use std::collections::hash_map::{Entry};
use std::collections::HashMap;
use std::hash::{Hasher};

use rand::seq::SliceRandom;

struct Bucket(Vec<i32>);

type Elem = i32;
type Idx = usize;

struct RandomizedSet {
    backing_map: HashMap<Elem, Idx>,
    entries: Vec<Elem>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * Based on {https://leetcode.com/problems/insert-delete-getrandom-o1/discuss/695890/Rust-cheapest-and-best}
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            backing_map: HashMap::new(),
            entries: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        match self.backing_map.entry(val) {
            Entry::Occupied(x) => {
                false
            }
            Entry::Vacant(x) => {
                x.insert(self.entries.len());
                self.entries.push(val);
                true
            }
        }
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        match self.backing_map.remove(&val) {
            None => {
                false
            }
            Some(idx) => {
                self.entries.swap_remove(idx);
                if let Some(v) = self.entries.get(idx) {
                    self.backing_map.insert(*v, idx);
                }
                true
            }
        }
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        *self.entries.choose(&mut rand::thread_rng()).unwrap()
    }
}
