use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The type of source element represented by a graph node.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum NodeKind {
    /// A class, interface, struct, service, repository, or controller.
    Type,
    /// A database, ORM, or persistence entity.
    Entity,
    /// A source file container.
    File,
}

impl NodeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Type => "type",
            Self::Entity => "entity",
            Self::File => "file",
        }
    }
}

/// The relationship represented by a graph edge.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EdgeKind {
    /// Source file or type imports another symbol.
    Imports,
    /// A type extends another type.
    Extends,
    /// A type implements an interface.
    Implements,
    /// An ORM model owns a relation to another model.
    RelatesTo,
    /// A file contains a type.
    Contains,
}

impl EdgeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Imports => "imports",
            Self::Extends => "extends",
            Self::Implements => "implements",
            Self::RelatesTo => "relates_to",
            Self::Contains => "contains",
        }
    }
}

/// A node in Apex's property graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// Stable graph identifier.
    pub id: String,
    /// Human-readable node name.
    pub name: String,
    /// Node category.
    pub kind: NodeKind,
    /// Relative source path when known.
    pub path: String,
    /// Optional architectural layer.
    pub layer: Option<String>,
}

/// A directed graph edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Relationship category.
    pub kind: EdgeKind,
}

/// A detected architecture or consistency violation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Violation {
    /// Stable violation rule id.
    pub rule_id: String,
    /// Human-readable message.
    pub message: String,
    /// Node id or file path that triggered the violation.
    pub subject: String,
}

/// In-memory property graph used by the parser, rules, layout, UI, and CLI.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Graph {
    /// Nodes keyed by stable id.
    pub nodes: BTreeMap<String, Node>,
    /// Directed graph edges.
    pub edges: Vec<Edge>,
}

impl Graph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a node by id.
    pub fn upsert_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Inserts a directed edge if it is not already present.
    pub fn add_edge(&mut self, from: impl Into<String>, to: impl Into<String>, kind: EdgeKind) {
        let edge = Edge {
            from: from.into(),
            to: to.into(),
            kind,
        };
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }

    /// Returns all direct neighbours of a node, regardless of edge direction.
    pub fn neighbours(&self, node_id: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for edge in &self.edges {
            if edge.from == node_id {
                out.insert(edge.to.clone());
            }
            if edge.to == node_id {
                out.insert(edge.from.clone());
            }
        }
        out
    }

    /// Returns the N-hop focus subgraph around a selected node.
    pub fn focus(&self, node_id: &str, hops: usize) -> Self {
        let mut seen = BTreeSet::from([node_id.to_string()]);
        let mut queue = VecDeque::from([(node_id.to_string(), 0usize)]);
        while let Some((current, depth)) = queue.pop_front() {
            if depth == hops {
                continue;
            }
            for next in self.neighbours(&current) {
                if seen.insert(next.clone()) {
                    queue.push_back((next, depth + 1));
                }
            }
        }
        let nodes = self
            .nodes
            .iter()
            .filter(|(id, _)| seen.contains(*id))
            .map(|(id, node)| (id.clone(), node.clone()))
            .collect();
        let edges = self
            .edges
            .iter()
            .filter(|edge| seen.contains(&edge.from) && seen.contains(&edge.to))
            .cloned()
            .collect();
        Self { nodes, edges }
    }

    /// Serializes graph data into deterministic JSON without external dependencies.
    pub fn to_json(&self) -> String {
        let mut nodes = String::new();
        for (index, node) in self.nodes.values().enumerate() {
            if index > 0 {
                nodes.push(',');
            }
            nodes.push_str(&format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"kind\":\"{}\",\"path\":\"{}\",\"layer\":{}}}",
                escape_json(&node.id),
                escape_json(&node.name),
                node.kind.as_str(),
                escape_json(&node.path),
                node.layer
                    .as_ref()
                    .map(|layer| format!("\"{}\"", escape_json(layer)))
                    .unwrap_or_else(|| "null".to_string())
            ));
        }
        let mut edges = String::new();
        for (index, edge) in self.edges.iter().enumerate() {
            if index > 0 {
                edges.push(',');
            }
            edges.push_str(&format!(
                "{{\"from\":\"{}\",\"to\":\"{}\",\"kind\":\"{}\"}}",
                escape_json(&edge.from),
                escape_json(&edge.to),
                edge.kind.as_str()
            ));
        }
        format!("{{\"nodes\":[{}],\"edges\":[{}]}}", nodes, edges)
    }

    /// Serializes graph data into Mermaid class-diagram syntax.
    pub fn to_mermaid(&self) -> String {
        let mut out = String::from("classDiagram\n");
        for node in self.nodes.values() {
            if matches!(node.kind, NodeKind::Type | NodeKind::Entity) {
                out.push_str(&format!("  class {}\n", sanitize_mermaid(&node.name)));
            }
        }
        for edge in &self.edges {
            if let (Some(from), Some(to)) = (self.nodes.get(&edge.from), self.nodes.get(&edge.to)) {
                let arrow = match edge.kind {
                    EdgeKind::Extends => "--|>",
                    EdgeKind::Implements => "..|>",
                    EdgeKind::RelatesTo => "-->",
                    EdgeKind::Imports | EdgeKind::Contains => "..>",
                };
                out.push_str(&format!(
                    "  {} {} {}\n",
                    sanitize_mermaid(&from.name),
                    arrow,
                    sanitize_mermaid(&to.name)
                ));
            }
        }
        out
    }

    /// Serializes graph data into an accessible SVG.
    pub fn to_svg(&self) -> String {
        let layout = layered_layout(self);
        let width = 260usize.max(layout.len() * 180 + 40);
        let height = 240usize.max(layout.values().map(|(_, y)| *y).max().unwrap_or(0) + 100);
        let mut out = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-labelledby=\"title\" viewBox=\"0 0 {} {}\"><title id=\"title\">Apex architecture diagram</title>",
            width, height
        );
        for edge in &self.edges {
            if let (Some((x1, y1)), Some((x2, y2))) = (layout.get(&edge.from), layout.get(&edge.to))
            {
                out.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#637083\" stroke-width=\"2\" />",
                    x1 + 60,
                    y1 + 24,
                    x2 + 60,
                    y2 + 24
                ));
            }
        }
        for node in self.nodes.values() {
            if let Some((x, y)) = layout.get(&node.id) {
                out.push_str(&format!(
                    "<g tabindex=\"0\" aria-label=\"{}\"><rect x=\"{}\" y=\"{}\" width=\"120\" height=\"48\" rx=\"8\" fill=\"#f7fbff\" stroke=\"#1f6feb\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\">{}</text></g>",
                    escape_xml(&node.name),
                    x,
                    y,
                    x + 60,
                    y + 29,
                    escape_xml(&node.name)
                ));
            }
        }
        out.push_str("</svg>");
        out
    }
}

/// Returns a deterministic layered layout for the graph.
pub fn layered_layout(graph: &Graph) -> BTreeMap<String, (usize, usize)> {
    let mut ids: Vec<_> = graph.nodes.keys().cloned().collect();
    ids.sort();
    ids.into_iter()
        .enumerate()
        .map(|(index, id)| {
            let x = 40 + (index % 4) * 180;
            let y = 40 + (index / 4) * 110;
            (id, (x, y))
        })
        .collect()
}

/// Parses a repository into a graph using lightweight language recognizers.
pub fn parse_repository(root: &Path) -> io::Result<Graph> {
    let mut graph = Graph::new();
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    for path in &files {
        let Some(ext) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };
        let content = fs::read_to_string(path)?;
        let relative = relative_path(root, path);
        match ext {
            "ts" | "tsx" | "js" | "jsx" => parse_typescript(&mut graph, &relative, &content),
            "py" => parse_python(&mut graph, &relative, &content),
            "java" => parse_java(&mut graph, &relative, &content),
            "prisma" => parse_prisma(&mut graph, &relative, &content),
            _ => {}
        }
    }
    Ok(graph)
}

/// Detects architecture rules and import cycles in a graph.
pub fn check_graph(graph: &Graph) -> Vec<Violation> {
    let mut violations = Vec::new();
    for edge in &graph.edges {
        if !matches!(edge.kind, EdgeKind::Imports) {
            continue;
        }
        let Some(from) = graph.nodes.get(&edge.from) else {
            continue;
        };
        let Some(to) = graph.nodes.get(&edge.to) else {
            continue;
        };
        if from.layer.as_deref() == Some("api") && to.layer.as_deref() == Some("infrastructure") {
            violations.push(Violation {
                rule_id: "RULE-LAYER-001".to_string(),
                message: format!(
                    "api layer '{}' must not import infrastructure '{}'",
                    from.name, to.name
                ),
                subject: from.path.clone(),
            });
        }
    }
    let import_edges: Vec<_> = graph
        .edges
        .iter()
        .filter(|edge| matches!(edge.kind, EdgeKind::Imports))
        .collect();
    for edge in &import_edges {
        if has_path(graph, &edge.to, &edge.from, &mut BTreeSet::new()) {
            violations.push(Violation {
                rule_id: "RULE-CYCLE-001".to_string(),
                message: format!(
                    "import cycle detected between '{}' and '{}'",
                    edge.from, edge.to
                ),
                subject: edge.from.clone(),
            });
        }
    }
    violations.sort_by(|left, right| {
        left.rule_id
            .cmp(&right.rule_id)
            .then(left.subject.cmp(&right.subject))
    });
    violations
        .dedup_by(|left, right| left.rule_id == right.rule_id && left.subject == right.subject);
    violations
}

fn collect_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if root.is_file() {
        out.push(root.to_path_buf());
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == "target" || name == "node_modules" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn parse_typescript(graph: &mut Graph, path: &str, content: &str) {
    let file_id = format!("file:{path}");
    graph.upsert_node(Node {
        id: file_id.clone(),
        name: path.to_string(),
        kind: NodeKind::File,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    for line in content.lines() {
        if let Some(name) =
            after_keyword(line, "class").or_else(|| after_keyword(line, "interface"))
        {
            let node_id = format!("type:{name}");
            graph.upsert_node(Node {
                id: node_id.clone(),
                name: name.clone(),
                kind: NodeKind::Type,
                path: path.to_string(),
                layer: layer_for_path(path),
            });
            graph.add_edge(file_id.clone(), node_id.clone(), EdgeKind::Contains);
            if let Some(parent) = after_keyword(line, "extends") {
                graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
            }
            if let Some(interface) = after_keyword(line, "implements") {
                graph.add_edge(node_id, format!("type:{interface}"), EdgeKind::Implements);
            }
        }
        if line.trim_start().starts_with("import ") {
            for imported in imported_symbols(line) {
                let imported_id = format!("type:{imported}");
                graph.upsert_node(Node {
                    id: imported_id.clone(),
                    name: imported.clone(),
                    kind: NodeKind::Type,
                    path: imported.clone(),
                    layer: layer_for_import(line),
                });
                graph.add_edge(file_id.clone(), imported_id, EdgeKind::Imports);
            }
        }
    }
}

fn parse_python(graph: &mut Graph, path: &str, content: &str) {
    let file_id = format!("file:{path}");
    graph.upsert_node(Node {
        id: file_id.clone(),
        name: path.to_string(),
        kind: NodeKind::File,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    for line in content.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("class ") {
            let name = rest
                .split(['(', ':'])
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            if name.is_empty() {
                continue;
            }
            let node_id = format!("type:{name}");
            graph.upsert_node(Node {
                id: node_id.clone(),
                name: name.clone(),
                kind: if content.contains("models.Model") {
                    NodeKind::Entity
                } else {
                    NodeKind::Type
                },
                path: path.to_string(),
                layer: layer_for_path(path),
            });
            graph.add_edge(file_id.clone(), node_id.clone(), EdgeKind::Contains);
            if let Some(parent) = rest
                .split('(')
                .nth(1)
                .and_then(|value| value.split(')').next())
            {
                let parent = parent.trim();
                if !parent.is_empty() {
                    graph.add_edge(node_id, format!("type:{parent}"), EdgeKind::Extends);
                }
            }
        }
        if trimmed.contains("ForeignKey(") {
            let target = trimmed
                .split("ForeignKey(")
                .nth(1)
                .and_then(|value| value.split([',', ')']).next())
                .unwrap_or_default()
                .trim_matches(['"', '\'', ' ']);
            if !target.is_empty() {
                graph.add_edge(
                    file_id.clone(),
                    format!("type:{target}"),
                    EdgeKind::RelatesTo,
                );
            }
        }
    }
}

fn parse_java(graph: &mut Graph, path: &str, content: &str) {
    let file_id = format!("file:{path}");
    graph.upsert_node(Node {
        id: file_id.clone(),
        name: path.to_string(),
        kind: NodeKind::File,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    for line in content.lines() {
        if let Some(name) =
            after_keyword(line, "class").or_else(|| after_keyword(line, "interface"))
        {
            let entity = content.contains("@Entity");
            let node_id = format!("type:{name}");
            graph.upsert_node(Node {
                id: node_id.clone(),
                name: name.clone(),
                kind: if entity {
                    NodeKind::Entity
                } else {
                    NodeKind::Type
                },
                path: path.to_string(),
                layer: layer_for_path(path),
            });
            graph.add_edge(file_id.clone(), node_id.clone(), EdgeKind::Contains);
            if let Some(interface) = after_keyword(line, "implements") {
                graph.add_edge(node_id, format!("type:{interface}"), EdgeKind::Implements);
            }
        }
        if line.trim_start().starts_with("import ") {
            let imported = line
                .trim()
                .trim_start_matches("import ")
                .trim_end_matches(';')
                .rsplit('.')
                .next()
                .unwrap_or_default();
            if !imported.is_empty() {
                graph.add_edge(
                    file_id.clone(),
                    format!("type:{imported}"),
                    EdgeKind::Imports,
                );
            }
        }
    }
}

fn parse_prisma(graph: &mut Graph, path: &str, content: &str) {
    let mut current: Option<String> = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed
            .strip_prefix("model ")
            .and_then(|rest| rest.split_whitespace().next())
        {
            let node_id = format!("type:{name}");
            current = Some(node_id.clone());
            graph.upsert_node(Node {
                id: node_id,
                name: name.to_string(),
                kind: NodeKind::Entity,
                path: path.to_string(),
                layer: Some("data".to_string()),
            });
        } else if trimmed == "}" {
            current = None;
        } else if let Some(source) = &current {
            let fields: Vec<_> = trimmed.split_whitespace().collect();
            if fields.len() >= 2 && fields[1].chars().next().is_some_and(char::is_uppercase) {
                graph.add_edge(
                    source.clone(),
                    format!("type:{}", fields[1].trim_end_matches("[]")),
                    EdgeKind::RelatesTo,
                );
            }
        }
    }
}

fn imported_symbols(line: &str) -> Vec<String> {
    if let Some(start) = line.find('{') {
        if let Some(end) = line[start + 1..].find('}') {
            return line[start + 1..start + 1 + end]
                .split(',')
                .filter_map(|part| part.split_whitespace().next())
                .filter(|part| !part.is_empty())
                .map(ToOwned::to_owned)
                .collect();
        }
    }
    line.split_whitespace()
        .nth(1)
        .filter(|value| value.chars().next().is_some_and(char::is_uppercase))
        .map(|value| vec![value.trim_matches(['{', '}', ',']).to_string()])
        .unwrap_or_default()
}

fn after_keyword(line: &str, keyword: &str) -> Option<String> {
    let parts: Vec<_> = line.split_whitespace().collect();
    let index = parts.iter().position(|part| *part == keyword)?;
    let candidate = parts.get(index + 1)?;
    let name = candidate
        .trim_matches(['{', '}', '(', ')', ':', ',', ';'])
        .split(['<', '(', '{'])
        .next()
        .unwrap_or_default()
        .to_string();
    (!name.is_empty()).then_some(name)
}

fn has_path(graph: &Graph, current: &str, target: &str, seen: &mut BTreeSet<String>) -> bool {
    if current == target {
        return true;
    }
    if !seen.insert(current.to_string()) {
        return false;
    }
    graph
        .edges
        .iter()
        .filter(|edge| matches!(edge.kind, EdgeKind::Imports) && edge.from == current)
        .any(|edge| has_path(graph, &edge.to, target, seen))
}

fn layer_for_path(path: &str) -> Option<String> {
    if path.contains("/api/") || path.starts_with("api/") {
        Some("api".to_string())
    } else if path.contains("/infrastructure/") || path.starts_with("infrastructure/") {
        Some("infrastructure".to_string())
    } else if path.contains("Repository") {
        Some("data".to_string())
    } else if path.contains("Service") {
        Some("service".to_string())
    } else {
        None
    }
}

fn layer_for_import(line: &str) -> Option<String> {
    if line.contains("infrastructure") {
        Some("infrastructure".to_string())
    } else if line.contains("api") {
        Some("api".to_string())
    } else {
        None
    }
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn sanitize_mermaid(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_one_hop_returns_neighbourhood() {
        let mut graph = Graph::new();
        graph.upsert_node(Node {
            id: "a".into(),
            name: "A".into(),
            kind: NodeKind::Type,
            path: "a.ts".into(),
            layer: None,
        });
        graph.upsert_node(Node {
            id: "b".into(),
            name: "B".into(),
            kind: NodeKind::Type,
            path: "b.ts".into(),
            layer: None,
        });
        graph.upsert_node(Node {
            id: "c".into(),
            name: "C".into(),
            kind: NodeKind::Type,
            path: "c.ts".into(),
            layer: None,
        });
        graph.add_edge("a", "b", EdgeKind::Imports);
        graph.add_edge("b", "c", EdgeKind::Imports);

        let focused = graph.focus("a", 1);

        assert!(focused.nodes.contains_key("a"));
        assert!(focused.nodes.contains_key("b"));
        assert!(!focused.nodes.contains_key("c"));
    }

    #[test]
    fn test_check_graph_detects_layer_violation() {
        let mut graph = Graph::new();
        graph.upsert_node(Node {
            id: "file:api/TokenValidator.ts".into(),
            name: "TokenValidator.ts".into(),
            kind: NodeKind::File,
            path: "api/TokenValidator.ts".into(),
            layer: Some("api".into()),
        });
        graph.upsert_node(Node {
            id: "type:CacheStore".into(),
            name: "CacheStore".into(),
            kind: NodeKind::Type,
            path: "infrastructure/CacheStore.ts".into(),
            layer: Some("infrastructure".into()),
        });
        graph.add_edge(
            "file:api/TokenValidator.ts",
            "type:CacheStore",
            EdgeKind::Imports,
        );

        let violations = check_graph(&graph);

        assert_eq!(violations[0].rule_id, "RULE-LAYER-001");
    }

    #[test]
    fn test_mermaid_output_contains_class_diagram() {
        let mut graph = Graph::new();
        graph.upsert_node(Node {
            id: "type:User".into(),
            name: "User".into(),
            kind: NodeKind::Entity,
            path: "schema.prisma".into(),
            layer: None,
        });

        let output = graph.to_mermaid();

        assert!(output.contains("classDiagram"));
        assert!(output.contains("class User"));
    }
}
