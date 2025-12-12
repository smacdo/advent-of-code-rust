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

    pub fn count(&self, v: &T) -> usize {
        *self.counters.get(v).unwrap_or(&0)
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
    fn count_elements() {
        let mut c: Counter<char> = Default::default();
        assert_eq!(c.count(&'a'), 0);
        assert_eq!(c.count(&'b'), 0);

        c.add('a');
        assert_eq!(c.count(&'a'), 1);
        assert_eq!(c.count(&'b'), 0);

        c.add('a');
        assert_eq!(c.count(&'a'), 2);
        assert_eq!(c.count(&'b'), 0);
    }
}
