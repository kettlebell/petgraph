
use ::{
    Direction,
    Incoming,
};

use visit::{
    GraphBase,
    GraphRef,
    IntoNodeIdentifiers,
    IntoNodeReferences,
    IntoNeighbors,
    IntoNeighborsDirected,
    IntoEdgeReferences,
    NodeCompactIndexable,
    NodeCount,
    NodeIndexable,
    Visitable,
    EdgeRef,
    Data,
    GraphProp,
};

/// An edge-reversing graph adaptor.
///
/// All edges have the opposite direction with `Reversed`.
#[derive(Copy, Clone, Debug)]
pub struct Reversed<G>(pub G);

impl<G: GraphBase> GraphBase for Reversed<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: GraphRef> GraphRef for Reversed<G> { }

Data!{delegate_impl [[G], G, Reversed<G>, access0]}

impl<G> IntoNeighbors for Reversed<G>
    where G: IntoNeighborsDirected
{
    type Neighbors = G::NeighborsDirected;
    fn neighbors(self, n: G::NodeId) -> G::NeighborsDirected
    {
        self.0.neighbors_directed(n, Incoming)
    }
}

impl<G> IntoNeighborsDirected for Reversed<G>
    where G: IntoNeighborsDirected
{
    type NeighborsDirected = G::NeighborsDirected;
    fn neighbors_directed(self, n: G::NodeId, d: Direction)
        -> G::NeighborsDirected
    {
        self.0.neighbors_directed(n, d.opposite())
    }
}

impl<G: Visitable> Visitable for Reversed<G>
{
    type Map = G::Map;
    fn visit_map(&self) -> G::Map {
        self.0.visit_map()
    }
    fn reset_map(&self, map: &mut Self::Map) {
        self.0.reset_map(map);
    }
}


/// A reversed edge reference
#[derive(Copy, Clone, Debug)]
pub struct ReversedEdgeReference<R>(R);

/// An edge reference
impl<R> EdgeRef for ReversedEdgeReference<R>
    where R: EdgeRef,
{
    type NodeId = R::NodeId;
    type EdgeId = R::EdgeId;
    type Weight = R::Weight;
    fn source(&self) -> Self::NodeId {
        self.0.target()
    }
    fn target(&self) -> Self::NodeId {
        self.0.source()
    }
    fn weight(&self) -> &Self::Weight {
        self.0.weight()
    }
    fn id(&self) -> Self::EdgeId {
        self.0.id()
    }
}

impl<G> IntoEdgeReferences for Reversed<G>
    where G: IntoEdgeReferences
{
    type EdgeRef = ReversedEdgeReference<G::EdgeRef>;
    type EdgeReferences = ReversedEdgeReferences<G::EdgeReferences>;
    fn edge_references(self) -> Self::EdgeReferences {
        ReversedEdgeReferences {
            iter: self.0.edge_references(),
        }
    }
}

/// A reversed edge references iterator.
pub struct ReversedEdgeReferences<I> {
    iter: I,
}

impl<I> Iterator for ReversedEdgeReferences<I>
    where I: Iterator,
          I::Item: EdgeRef,
{
    type Item = ReversedEdgeReference<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ReversedEdgeReference)
    }
}

macro_rules! access0 {
    ($e:expr) => ($e.0)
}

NodeIndexable!{delegate_impl [[G], G, Reversed<G>, access0]}
NodeCompactIndexable!{delegate_impl [[G], G, Reversed<G>, access0]}
IntoNodeIdentifiers!{delegate_impl [[G], G, Reversed<G>, access0]}
IntoNodeReferences!{delegate_impl [[G], G, Reversed<G>, access0]}
GraphProp!{delegate_impl [[G], G, Reversed<G>, access0]}
NodeCount!{delegate_impl [[G], G, Reversed<G>, access0]}


