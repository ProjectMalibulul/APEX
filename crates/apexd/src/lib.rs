use std::io;
use std::path::Path;

/// Daemon status returned by the local development server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaemonStatus {
    /// Workspace path being served.
    pub workspace: String,
    /// Number of parsed nodes currently available.
    pub nodes: usize,
}

/// Starts one deterministic scan cycle for the daemon.
pub fn serve_once(workspace: impl AsRef<Path>) -> io::Result<DaemonStatus> {
    let graph = apex_core::parse_repository(workspace.as_ref())?;
    Ok(DaemonStatus {
        workspace: workspace.as_ref().to_string_lossy().to_string(),
        nodes: graph.nodes.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serve_once_current_dir_succeeds() {
        let status = serve_once(".").expect("serve once");
        assert!(!status.workspace.is_empty());
    }
}
