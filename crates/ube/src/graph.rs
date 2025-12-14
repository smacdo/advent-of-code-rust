use std::collections::{HashMap, HashSet};

use slotmap::{SecondaryMap, SlotMap};

#[derive(Debug, PartialEq)]
struct Node {
    id: usize,
    edges: Vec<NodeKey>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            edges: Default::default(),
        }
    }
}

slotmap::new_key_type! { pub struct NodeKey; }

#[derive(Debug)]
pub struct Graph {
    nodes: SlotMap<NodeKey, Node>,
    node_to_name: SecondaryMap<NodeKey, String>, // TODO: make this optional, for debugging.
    _next_id: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            node_to_name: Default::default(),
            _next_id: 1,
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        if self.nodes.keys().len() == other.nodes.keys().len() {
            for k in self.nodes.keys() {
                if self.nodes[k] != other.nodes[k] {
                    return false;
                }
            }
        }

        true
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ensure all of the nodes have a name for printing.
        let mut names = self.node_to_name.clone();

        for k in self.nodes.keys() {
            if !names.contains_key(k) {
                names[k] = format!("${}", self.nodes[k].id);
            }
        }

        // Sort node in alphabetical order for predictable output.
        let mut sorted_nodes = names.iter().collect::<Vec<_>>();
        let names_len = sorted_nodes.len();

        sorted_nodes.sort_by_key(|n| n.1);

        writeln!(f, "{{")?;

        for (node_index, (node_key, node_name)) in sorted_nodes.into_iter().enumerate() {
            let edges = &self.nodes[node_key].edges;

            write!(f, "\t\"{node_name}\": [")?;

            for (edge_index, edge_nk) in edges.into_iter().enumerate() {
                write!(
                    f,
                    "\"{}{}\"",
                    names.get(*edge_nk).unwrap(),
                    if edge_index < edges.len() - 1 {
                        ","
                    } else {
                        ""
                    }
                )?;
            }

            writeln!(f, "]{}", if node_index < names_len - 1 { "," } else { "" })?;
        }

        writeln!(f, "}}")
    }
}

pub struct NodeBuilder {
    name: Option<String>,
    unidir_edges: Vec<String>,
    bidir_edges: Vec<String>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            unidir_edges: Default::default(),
            bidir_edges: Default::default(),
        }
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }

    pub fn add_edge<S: Into<String>>(&mut self, to_node_name: S) {
        self.unidir_edges.push(to_node_name.into());
    }

    pub fn add_bidir_edge<S: Into<String>>(&mut self, to_node_name: S) {
        self.bidir_edges.push(to_node_name.into());
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.set_name(name);
        self
    }

    pub fn with_edge<S: Into<String>>(mut self, to_node_name: S) -> Self {
        self.add_edge(to_node_name);
        self
    }

    pub fn with_bidir_edge<S: Into<String>>(mut self, to_node_name: S) -> Self {
        self.add_bidir_edge(to_node_name);
        self
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GraphBuilder {
    nodes: SlotMap<NodeKey, Node>,
    name_to_node: HashMap<String, NodeKey>,
    node_to_name: SecondaryMap<NodeKey, String>,
    next_id: usize,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            name_to_node: Default::default(),
            node_to_name: Default::default(),
            next_id: 1,
        }
    }

    pub fn add_node(&mut self, n_builder: NodeBuilder) {
        let nk = self.register_node(n_builder.name);

        for to_node_name in n_builder.unidir_edges {
            let to_nk = self.register_node(Some(to_node_name));
            self.add_edge(nk, to_nk);
        }

        for to_node_name in n_builder.bidir_edges {
            let to_nk = self.register_node(Some(to_node_name));
            self.add_edge(nk, to_nk);
            self.add_edge(to_nk, nk);
        }
    }

    pub fn with_node<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(NodeBuilder) -> NodeBuilder,
    {
        self.add_node(builder_fn(NodeBuilder::new()));
        self
    }

    fn add_edge(&mut self, a: NodeKey, b: NodeKey) {
        if !self.nodes[a].edges.contains(&b) {
            self.nodes[a].edges.push(b);
        }
    }

    fn register_node(&mut self, name: Option<String>) -> NodeKey {
        // Nodes with a name can only be registered once because duplicate names are not supported.
        if let Some(name) = &name {
            if self.name_to_node.contains_key(name) {
                return self.name_to_node[name];
            }
        }

        // Create the node and cache the name -> node and node -> name look up to speed up future
        // look ups.
        let nk = self.nodes.insert(Node::new(self.next_id));
        self.next_id += 1;

        if let Some(name) = name {
            self.name_to_node.insert(name.clone().to_string(), nk);
            self.node_to_name.insert(nk, name.to_string());
        }

        nk
    }

    pub fn build(self) -> (Graph, HashMap<String, NodeKey>) {
        (
            Graph {
                nodes: self.nodes,
                node_to_name: self.node_to_name,
                _next_id: self.next_id,
            },
            self.name_to_node,
        )
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
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
        statuses.insert(nk, VisitStatus::New);
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

    for to_k in &g.nodes[nk].edges {
        if statuses[*to_k] == VisitStatus::Active
            || (statuses[*to_k] == VisitStatus::New && !is_acyclic_dfs(*to_k, g, statuses))
        {
            return false;
        }
    }

    statuses[nk] = VisitStatus::Finished;
    true
}
