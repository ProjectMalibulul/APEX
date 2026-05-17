/// A parsed result returned by a WASM or native parser plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginResult {
    /// Plugin name.
    pub plugin: String,
    /// Number of nodes contributed by the plugin.
    pub nodes: usize,
}

/// Validates a plugin export name against the Apex ABI contract.
pub fn validate_export(name: &str) -> bool {
    matches!(name, "apex_plugin_version" | "apex_parse" | "apex_free")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_export_accepts_required_entrypoint() {
        assert!(validate_export("apex_parse"));
        assert!(!validate_export("malloc"));
    }
}
