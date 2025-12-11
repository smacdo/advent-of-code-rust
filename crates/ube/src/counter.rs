use std::{collections::HashMap, hash::Hash};

pub struct Counter<T> {
    counters: HashMap<T, usize>,
}

impl<T> Counter<T> {
    pub fn new() -> Self {
        Self {
            counters: Default::default(),
        }
    }
}

impl<T> Counter<T>
where
    T: Clone,
{
    pub fn most_common(&self) -> Vec<(T, usize)> {
        let mut elements = self
            .counters
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>();
        elements.sort_by_key(|a| std::cmp::Reverse(a.1));
        elements
    }
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    pub fn add(&mut self, v: T) -> usize {
        let count = self.counters.entry(v).or_default();
        *count += 1;

        *count
    }
}

impl<T> Default for Counter<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_of_first_test_function() {}
}
