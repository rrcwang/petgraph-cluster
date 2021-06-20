use core::hash::Hash;
use petgraph::{
    graph::Edges,
    visit::{EdgeRef, GraphBase, IntoEdges, Visitable},
    Directed, Graph,
};
use std::cell::RefCell;
use std::rc::Weak;

/// Represents a subgraph of a graph.
struct Subgraph<N, E> {
    /// Weak reference to the graph from which the subgraph is taken.
    ///
    /// For our purposes, this will be referred to as the _parent graph_.
    parent_graph: Option<Weak<RefCell<Graph<N, E>>>>,

    /// Nodes of the subgraph, as `NodeId` indices into the parent graph.
    nodes: Vec<<Graph<N, E> as GraphBase>::NodeId>,

    /// Edges of the subgraph, as `EdgeId` indices into the parent graph.
    edges: Vec<<Graph<N, E> as GraphBase>::EdgeId>,
}

impl<N, E> Subgraph<N, E> {
    pub fn new() -> Subgraph<N, E> {
        Subgraph {
            parent_graph: None,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn parent_graph(&self) -> &Option<Weak<RefCell<Graph<N, E>>>> {
        &self.parent_graph
    }
    pub fn nodes(&self) -> &Vec<<Graph<N, E> as GraphBase>::NodeId> {
        &self.nodes
    }
    pub fn edges(&self) -> &Vec<<Graph<N, E> as GraphBase>::EdgeId> {
        &self.edges
    }
}

/// Represents a graph partition.
pub struct GraphPartition<N, E> {
    /// Weak reference to the graph $G$ for which partitions $G_i$ are made.
    ///
    /// The graph must be _connected_, i.e. there exists a path from between every pair of
    /// nodes in the graph.
    ///
    /// For our purposes, this will be referred to as the _parent graph_.
    graph: Weak<RefCell<Graph<N, E>>>,

    /// Disconnected subgraphs $G_i, i \in \{1,...,\N\}$ of the parent graph.
    subgraphs: Vec<Subgraph<N, E>>,

    /// Vertex cuts separating the subgraphs. There are are maximum of $N-1$ vertex cuts.
    cuts: Vec<<Graph<N, E> as GraphBase>::EdgeId>,
}

pub fn graph_cut_matrix<N, E>(graph: Graph<N, E>) -> () {}

/// Computes the sparsity of a cut.
///
/// # Parameters:
/// * `graph`   - a graph from 'petgraph'
/// * `cut`     - a vector containing graph edges as [`EdgeRef`](petgraph::visit::EdgeRef)
pub fn graph_cut_sparsity<G, Er>(graph: G, cut: Vec<Er>) -> f64
where
    G: IntoEdges + Visitable, // need be undirected?
    Er: EdgeRef,              // how best to represent cut??
{
    for e in cut {
        e.source();
    }

    0.0
}

#[cfg(test)]
mod test {
    use super::{GraphPartition, Subgraph};

    #[test]
    fn subgraph_init() {
        // let mut subgraph = Subgraph::new();
    }
}
