use std::io;
use std::path::Path;

pub use apex_core::{Edge, EdgeKind, Graph, Node, NodeKind};

/// Parses all supported source files below a repository root.
pub fn parse_workspace(root: impl AsRef<Path>) -> io::Result<Graph> {
    apex_core::parse_repository(root.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_workspace_typescript_class_detected() {
        let root = std::env::temp_dir().join(format!("apex-parser-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp fixture");
        fs::write(root.join("UserService.ts"), "export class UserService {}\n")
            .expect("write fixture");

        let graph = parse_workspace(&root).expect("parse workspace");

        assert!(graph.nodes.contains_key("type:UserService"));
        let _ = fs::remove_dir_all(root);
    }
}
