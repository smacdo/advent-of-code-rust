use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::counter::Counter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetId(usize);

impl std::fmt::Display for SetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Set Id#{}", self.0)
    }
}

/// An opaque handle for elements stored in a disjoint set.
impl SetId {
    pub(crate) fn value(&self) -> usize {
        self.0
    }
}

pub struct Node {
    parent: usize,
    size: usize,
}

pub struct UnionFind<T> {
    index: HashMap<T, usize>,
    nodes: Vec<Node>,
}

impl<T> UnionFind<T> {
    /// Initialize as an empty union find with no elements.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    /// Check if the union find data structure is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get the number of elements in this union find data structure.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T> UnionFind<T>
where
    T: Hash + Eq,
{
    /// Add an element to the union find data structure, with the new element `v` belonging to its
    /// own distinct set.
    pub fn add(&mut self, v: T) -> SetId {
        let new_index = *self.index.entry(v).or_insert_with(|| {
            let next_index = self.nodes.len();

            self.nodes.push(Node {
                parent: next_index,
                size: 1,
            });

            next_index
        });

        SetId(new_index)
    }

    /// Determines the root of the set containing element `v`.
    pub fn find(&self, v: &T) -> SetId {
        let mut index = self.index.get(v).expect("value must exist in disjoint set");
        let mut node = &self.nodes[*index];

        while node.parent != *index {
            index = &node.parent;
            node = &self.nodes[*index]
        }

        SetId(*index)
    }

    /// Merge the sets containing the elements `a` and `b` into a single set.
    ///
    /// This method returns the root of the combined set, or `None` if the two elements are already
    /// in the same set.
    pub fn union(&mut self, a: &T, b: &T) -> Option<SetId> {
        let ai = self.find(a).value();
        let bi = self.find(b).value();

        // Skip the union operation if a and b belong to the same set.
        if ai == bi {
            return None;
        }

        // Swap a and b if needed to make sure `a` has the fewest nodes since
        // it will be the new root.
        let (ai, bi) = if self.nodes[ai].size <= self.nodes[bi].size {
            (ai, bi)
        } else {
            (bi, ai)
        };

        // Perform set union on x and y.
        self.nodes[bi].parent = ai;
        self.nodes[ai].size += self.nodes[bi].size;

        Some(SetId(ai))
    }

    /// Check if element `b` and `b` belong to the same set.
    pub fn is_connected(&self, _a: &T, _b: &T) -> bool {
        todo!("implement me! -- disjoint_set.rs:97");
    }
}

impl<T> UnionFind<T>
where
    T: Hash + Eq,
{
    /// Get a list of the sets in this disjoint set container including the number of elements in
    /// each set.
    pub fn sets(&self) -> Vec<(SetId, usize)> {
        let mut c: Counter<SetId> = Counter::new();

        for v in self.index.keys() {
            c.add(self.find(v));
        }

        c.most_common()
    }
}

impl<T> FromIterator<T> for UnionFind<T>
where
    T: Eq + Hash,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut index: HashMap<T, usize> = Default::default();
        let mut nodes: Vec<Node> = Default::default();

        iter.into_iter().enumerate().for_each(|(i, v)| {
            index.insert(v, nodes.len());
            nodes.push(Node { parent: i, size: 1 });
        });

        Self { index, nodes }
    }
}

impl<T, const N: usize> From<[T; N]> for UnionFind<T>
where
    T: Clone + Eq + Hash,
{
    fn from(value: [T; N]) -> Self {
        Self {
            index: HashMap::from_iter(value.iter().enumerate().map(|(i, v)| (v.clone(), i))),
            nodes: value
                .iter()
                .enumerate()
                .map(|(i, _)| Node { parent: i, size: 1 })
                .collect::<Vec<_>>(),
        }
    }
}

impl<T> Default for UnionFind<T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::spatial::Point3;

    use super::*;

    #[test]
    fn can_find_items_after_adding() {
        let mut s: UnionFind<Point3> = Default::default();

        let id_1 = s.add(Point3 { x: 1, y: 2, z: 3 });
        let id_2 = s.add(Point3 { x: 5, y: 8, z: 4 });

        assert_ne!(id_1, id_2);

        assert_eq!(s.find(&Point3 { x: 1, y: 2, z: 3 }), id_1);
        assert_eq!(s.find(&Point3 { x: 5, y: 8, z: 4 }), id_2);
    }

    #[test]
    fn len_tracks_number_of_nodes() {
        let mut s: UnionFind<Point3> = Default::default();
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());

        s.add(Point3 { x: 1, y: 2, z: 3 });
        assert_eq!(s.len(), 1);
        assert!(!s.is_empty());

        s.add(Point3 { x: 5, y: 8, z: 4 });
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[test]
    fn duplicates_return_same_id_and_are_not_added() {
        let mut s: UnionFind<Point3> = Default::default();

        let id_1a = s.add(Point3 { x: 1, y: 2, z: 3 });
        let id_2a = s.add(Point3 { x: 5, y: 8, z: 4 });

        assert_eq!(s.len(), 2);
        assert_ne!(id_1a, id_2a);

        let id_1b = s.add(Point3 { x: 1, y: 2, z: 3 });
        let id_2b = s.add(Point3 { x: 5, y: 8, z: 4 });

        assert_eq!(s.len(), 2);
        assert_eq!(id_1a, id_1b);
        assert_eq!(id_2a, id_2b);
    }

    #[test]
    fn union_elements() {
        let mut s: UnionFind<Point3> = Default::default();
        let a = Point3 {
            x: 162,
            y: 817,
            z: 812,
        };
        let b = Point3 {
            x: 425,
            y: 690,
            z: 689,
        };
        let c = Point3 {
            x: 431,
            y: 825,
            z: 988,
        };

        // Initial iteration - all three points are in unique sets.
        s.add(a);
        s.add(b);
        s.add(c);

        assert_ne!(s.find(&a), s.find(&b));
        assert_ne!(s.find(&b), s.find(&c));

        // First iteration - union a & b, with c being its own set.
        let id_u = s.union(&a, &b);

        assert_eq!(s.find(&a), id_u);
        assert_eq!(s.find(&b), id_u);
        assert_ne!(s.find(&a), s.find(&c));

        // Second iteration - union a & c. All points will be in the same set.
        let id_u = s.union(&a, &c);

        assert_eq!(s.find(&a), id_u);
        assert_eq!(s.find(&b), id_u);
        assert_eq!(s.find(&c), id_u);
    }
}
