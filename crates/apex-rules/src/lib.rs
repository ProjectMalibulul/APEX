pub use apex_core::Violation;

use apex_core::Graph;

/// Runs Apex's built-in architecture checks.
pub fn check(graph: &Graph) -> Vec<Violation> {
    apex_core::check_graph(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph_has_no_violations() {
        assert!(check(&Graph::new()).is_empty());
    }
}
