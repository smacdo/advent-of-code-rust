use std::collections::HashMap;

use slotmap::{SecondaryMap, SlotMap};

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    neighbors: Vec<NodeKey>,
}

impl Node {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            neighbors: Default::default(),
        }
    }
}

slotmap::new_key_type! { pub struct NodeKey; }

#[derive(Debug)]
pub struct Graph {
    nodes: SlotMap<NodeKey, Node>,
    names: HashMap<String, NodeKey>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            names: Default::default(),
        }
    }

    pub fn add_node_with_neighbors<S: Into<String> + Clone>(
        &mut self,
        name: S,
        neighbors: &[&str],
    ) -> NodeKey {
        // Create the node.
        // TODO: How to handle duplicates? Should this be an error, or should the existing node be
        //       re-used?
        let nk = self.nodes.insert(Node {
            name: name.clone().into(),
            neighbors: Default::default(),
        });

        self.names.insert(name.into(), nk);

        // Add edges to the provided neighbors.
        for e in neighbors {
            // Create the neighbor node if it doesn't exist yet. This simplifies graph creation by
            // not forcing callers to pre-create all nodes before adding them as edges.
            let dest_nk = self
                .names
                .entry(e.clone().to_string())
                .or_insert_with_key(|_| {
                    self.nodes.insert(Node {
                        name: (*e).into(),
                        neighbors: Default::default(),
                    })
                });

            self.nodes[nk].neighbors.push(*dest_nk);
            self.nodes[*dest_nk].neighbors.push(nk);
        }

        nk
    }

    pub fn add_bidir_edge(&mut self, a: NodeKey, b: NodeKey) {
        // TODO: Handle duplicate edges when adding.
        self.nodes[a].neighbors.push(b);
        self.nodes[b].neighbors.push(a);
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        if self.names != other.names {
            return false;
        }

        for nk in self.names.values() {
            if !other.nodes.contains_key(*nk) || self.nodes[*nk] != other.nodes[*nk] {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut names = self.names.keys().collect::<Vec<_>>();
        let names_len = names.len();

        names.sort();

        writeln!(f, "{{")?;

        for (i, n) in names.into_iter().enumerate() {
            write!(f, "\t\"{n}\": [")?;

            let neighbors = &self.nodes[self.names[n]].neighbors;

            for (i, w) in neighbors.into_iter().enumerate() {
                write!(
                    f,
                    "\"{}{}\"",
                    self.nodes[*w].name,
                    if i < neighbors.len() - 1 { "," } else { "" }
                )?;
            }

            writeln!(f, "]{}", if i < names_len - 1 { "," } else { "" })?;
        }

        writeln!(f, "}}")
    }
}

impl<'a> FromIterator<(&'a str, &'a [&'a str])> for Graph {
    fn from_iter<U: IntoIterator<Item = (&'a str, &'a [&'a str])>>(iter: U) -> Self {
        // TODO: Handle duplicate node names.
        let mut nodes: SlotMap<NodeKey, Node> = Default::default();
        let mut names: HashMap<String, NodeKey> = Default::default();

        for (node_name, edges) in iter.into_iter() {
            let from_key = nodes.insert(Node {
                name: node_name.to_string(),
                neighbors: Default::default(),
            });

            names.insert(node_name.to_string(), from_key);

            for e in edges {
                let to_key = names.entry(e.to_string()).or_insert_with_key(|_| {
                    nodes.insert(Node {
                        name: (*e).to_string(),
                        neighbors: Default::default(),
                    })
                });

                nodes[from_key].neighbors.push(*to_key);
                nodes[*to_key].neighbors.push(from_key);
            }
        }

        Self { nodes, names }
    }
}

#[derive(PartialEq)]
pub enum VisitStatus {
    New,
    Active,
    Finished,
}

/// Checks if `g` is a directed graph with no cycles or returns false if a cycle is found.
pub fn is_acyclic(g: &Graph) -> bool {
    // Set initial status for all nodes to be new before beginning traversal.
    let mut statuses: SecondaryMap<NodeKey, VisitStatus> = Default::default();

    for nk in g.nodes.keys() {
        statuses[nk] = VisitStatus::New;
    }

    // Check if every node is acyclic.
    for nk in g.nodes.keys() {
        statuses[nk] = VisitStatus::New;

        if !is_acyclic_dfs(nk, g, &mut statuses) {
            return false;
        }
    }

    true
}

fn is_acyclic_dfs(
    nk: NodeKey,
    g: &Graph,
    statuses: &mut SecondaryMap<NodeKey, VisitStatus>,
) -> bool {
    statuses[nk] = VisitStatus::Active;

    for to_k in &g.nodes[nk].neighbors {
        if statuses[*to_k] == VisitStatus::Active
            || (statuses[*to_k] == VisitStatus::New && !is_acyclic_dfs(nk, g, statuses))
        {
            return false;
        }
    }

    statuses[nk] = VisitStatus::Finished;
    true
}
