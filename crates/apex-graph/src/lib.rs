pub use apex_core::{Edge, EdgeKind, Graph, Node, NodeKind};

/// Creates an empty Apex property graph.
pub fn empty_graph() -> Graph {
    Graph::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph_has_no_nodes() {
        assert!(empty_graph().nodes.is_empty());
    }
}
