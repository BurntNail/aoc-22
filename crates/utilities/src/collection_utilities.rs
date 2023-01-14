use std::{collections::HashSet, hash::Hash};

pub trait HashSetExt<T: Eq + Hash> {
    fn contains_any(&self, v: &[T]) -> bool;
}

impl<T: Eq + Hash> HashSetExt<T> for HashSet<T> {
    fn contains_any(&self, v: &[T]) -> bool {
        v.iter().any(|x| self.contains(x))
    }
}
