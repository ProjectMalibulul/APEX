use std::collections::BTreeMap;

use apex_core::Graph;

/// Computes deterministic coordinates for diagram rendering.
pub fn layout(graph: &Graph) -> BTreeMap<String, (usize, usize)> {
    apex_core::layered_layout(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_empty_graph_is_empty() {
        assert!(layout(&Graph::new()).is_empty());
    }
}
