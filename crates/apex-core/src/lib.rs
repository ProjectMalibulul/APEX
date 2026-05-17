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

/// A user-configurable architecture rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDefinition {
    /// Stable rule identifier.
    pub id: String,
    /// Rule type, such as `forbidden_import` or `import_cycle`.
    pub rule_type: String,
    /// Source layer for layer-based rules.
    pub from: Option<String>,
    /// Target layer for layer-based rules.
    pub to: Option<String>,
    /// Whether the rule should run.
    pub enabled: bool,
}

/// A language recognizer supported by the local parser.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageSupport {
    /// Human-readable language name.
    pub name: &'static str,
    /// File extensions recognized by Apex.
    pub extensions: &'static [&'static str],
    /// Major symbols extracted by the recognizer.
    pub extracts: &'static str,
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
        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let content = fs::read_to_string(path)?;
        let relative = relative_path(root, path);
        match ext {
            "ts" | "tsx" | "js" | "jsx" => parse_typescript(&mut graph, &relative, &content),
            "py" => parse_python(&mut graph, &relative, &content),
            "java" => parse_java(&mut graph, &relative, &content),
            "go" => parse_go(&mut graph, &relative, &content),
            "rs" => parse_rust(&mut graph, &relative, &content),
            "kt" | "kts" => parse_kotlin(&mut graph, &relative, &content),
            "cs" => parse_csharp(&mut graph, &relative, &content),
            "sql" => parse_sql(&mut graph, &relative, &content),
            "prisma" => parse_prisma(&mut graph, &relative, &content),
            "json" | "toml" | "yaml" | "yml" => parse_manifest(&mut graph, &relative, &content),
            _ => {
                if relative.ends_with("go.mod")
                    || relative.ends_with("pom.xml")
                    || relative.ends_with("build.gradle")
                {
                    parse_manifest(&mut graph, &relative, &content);
                }
            }
        }
    }
    Ok(graph)
}

/// Returns the languages and file types recognized by the local parser.
pub fn supported_languages() -> Vec<LanguageSupport> {
    vec![
        LanguageSupport {
            name: "TypeScript / JavaScript",
            extensions: &["ts", "tsx", "js", "jsx"],
            extracts: "classes, interfaces, imports, extends, implements",
        },
        LanguageSupport {
            name: "Python",
            extensions: &["py"],
            extracts: "classes, base classes, Django-style model relations",
        },
        LanguageSupport {
            name: "Java",
            extensions: &["java"],
            extracts: "classes, interfaces, Spring/JPA annotations, imports, implements",
        },
        LanguageSupport {
            name: "Go",
            extensions: &["go"],
            extracts: "structs, interfaces, functions, imports",
        },
        LanguageSupport {
            name: "Rust",
            extensions: &["rs"],
            extracts: "structs, enums, traits, impls, use imports",
        },
        LanguageSupport {
            name: "Kotlin",
            extensions: &["kt", "kts"],
            extracts: "classes, interfaces, objects, imports, inheritance",
        },
        LanguageSupport {
            name: "C#",
            extensions: &["cs"],
            extracts: "classes, interfaces, structs, records, using imports, inheritance",
        },
        LanguageSupport {
            name: "Prisma",
            extensions: &["prisma"],
            extracts: "models and relations",
        },
        LanguageSupport {
            name: "SQL",
            extensions: &["sql"],
            extracts: "tables and foreign-key references",
        },
        LanguageSupport {
            name: "Manifests",
            extensions: &["json", "toml", "yaml", "yml"],
            extracts: "package/config files as graph context nodes",
        },
    ]
}

/// Returns Apex's built-in rule set.
pub fn default_rules() -> Vec<RuleDefinition> {
    vec![
        RuleDefinition {
            id: "RULE-LAYER-001".to_string(),
            rule_type: "forbidden_import".to_string(),
            from: Some("api".to_string()),
            to: Some("infrastructure".to_string()),
            enabled: true,
        },
        RuleDefinition {
            id: "RULE-CYCLE-001".to_string(),
            rule_type: "import_cycle".to_string(),
            from: None,
            to: None,
            enabled: true,
        },
    ]
}

/// Loads an `apex.rules.yaml` file using Apex's small documented rule schema.
pub fn load_rules(path: &Path) -> io::Result<Vec<RuleDefinition>> {
    let content = fs::read_to_string(path)?;
    let mut rules = Vec::new();
    let mut current: Option<RuleDefinition> = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- id:") {
            if let Some(rule) = current.take() {
                rules.push(rule);
            }
            current = Some(RuleDefinition {
                id: yaml_value(trimmed.trim_start_matches("- id:")),
                rule_type: String::new(),
                from: None,
                to: None,
                enabled: true,
            });
        } else if let Some(rule) = current.as_mut() {
            if let Some(value) = trimmed.strip_prefix("id:") {
                rule.id = yaml_value(value);
            } else if let Some(value) = trimmed.strip_prefix("type:") {
                rule.rule_type = yaml_value(value);
            } else if let Some(value) = trimmed.strip_prefix("from:") {
                rule.from = Some(yaml_value(value));
            } else if let Some(value) = trimmed.strip_prefix("to:") {
                rule.to = Some(yaml_value(value));
            } else if let Some(value) = trimmed.strip_prefix("enabled:") {
                rule.enabled = yaml_value(value) != "false";
            }
        }
    }
    if let Some(rule) = current {
        rules.push(rule);
    }
    Ok(rules
        .into_iter()
        .filter(|rule| !rule.id.is_empty() && !rule.rule_type.is_empty())
        .collect())
}

/// Detects architecture rules and import cycles in a graph.
pub fn check_graph(graph: &Graph) -> Vec<Violation> {
    check_graph_with_rules(graph, &default_rules())
}

/// Detects architecture violations using explicit rule definitions.
pub fn check_graph_with_rules(graph: &Graph, rules: &[RuleDefinition]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for rule in rules.iter().filter(|rule| rule.enabled) {
        if rule.rule_type == "forbidden_import" {
            let from_layer = rule.from.as_deref();
            let to_layer = rule.to.as_deref();
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
                if from.layer.as_deref() == from_layer && to.layer.as_deref() == to_layer {
                    violations.push(Violation {
                        rule_id: rule.id.clone(),
                        message: format!(
                            "{} layer '{}' must not import {} '{}'",
                            from_layer.unwrap_or("source"),
                            from.name,
                            to_layer.unwrap_or("target"),
                            to.name
                        ),
                        subject: from.path.clone(),
                    });
                }
            }
        } else if rule.rule_type == "import_cycle" {
            let import_edges: Vec<_> = graph
                .edges
                .iter()
                .filter(|edge| matches!(edge.kind, EdgeKind::Imports))
                .collect();
            for edge in &import_edges {
                if has_path(graph, &edge.to, &edge.from, &mut BTreeSet::new()) {
                    violations.push(Violation {
                        rule_id: rule.id.clone(),
                        message: format!(
                            "import cycle detected between '{}' and '{}'",
                            edge.from, edge.to
                        ),
                        subject: edge.from.clone(),
                    });
                }
            }
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

fn parse_go(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut in_import_block = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "import (" {
            in_import_block = true;
            continue;
        }
        if in_import_block && trimmed == ")" {
            in_import_block = false;
            continue;
        }
        if let Some(name) = go_type_name(trimmed, "type ", " struct")
            .or_else(|| go_type_name(trimmed, "type ", " interface"))
        {
            insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
        }
        if let Some(name) = trimmed
            .strip_prefix("func ")
            .and_then(|rest| rest.split('(').next())
        {
            if !name.is_empty() && name.chars().next().is_some_and(char::is_uppercase) {
                insert_type_node(graph, path, &file_id, name, NodeKind::Type);
            }
        }
        if trimmed.starts_with("import ") || in_import_block {
            let import_name = trimmed
                .trim_start_matches("import ")
                .trim_matches(['"', '`', '(', ')', ' '])
                .rsplit('/')
                .next()
                .unwrap_or_default();
            if !import_name.is_empty() {
                graph.add_edge(
                    file_id.clone(),
                    format!("type:{import_name}"),
                    EdgeKind::Imports,
                );
            }
        }
    }
}

fn parse_rust(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["struct", "enum", "trait"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
            }
        }
        if let Some(name) = after_keyword(trimmed, "impl") {
            let clean = name.split('<').next().unwrap_or(&name).trim();
            if !clean.is_empty() && clean != "for" {
                insert_type_node(graph, path, &file_id, clean, NodeKind::Type);
            }
        }
        if trimmed.starts_with("use ") {
            let imported = trimmed
                .trim_start_matches("use ")
                .trim_end_matches(';')
                .split("::")
                .next()
                .unwrap_or_default();
            if !imported.is_empty()
                && imported != "crate"
                && imported != "self"
                && imported != "super"
            {
                graph.add_edge(
                    file_id.clone(),
                    format!("type:{imported}"),
                    EdgeKind::Imports,
                );
            }
        }
    }
}

fn parse_kotlin(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["class", "interface", "object", "data"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
                if let Some(parent) = trimmed
                    .split(':')
                    .nth(1)
                    .and_then(|value| value.split(['(', '{', ',']).next())
                {
                    let parent = parent.trim();
                    if !parent.is_empty() {
                        graph.add_edge(
                            format!("type:{name}"),
                            format!("type:{parent}"),
                            EdgeKind::Extends,
                        );
                    }
                }
            }
        }
        if trimmed.starts_with("import ") {
            let imported = trimmed
                .trim_start_matches("import ")
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

fn parse_csharp(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["class", "interface", "struct", "record"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
                if let Some(parent_list) = trimmed.split(':').nth(1) {
                    for parent in parent_list.split(',') {
                        let parent = parent.split_whitespace().next().unwrap_or_default();
                        if !parent.is_empty() {
                            graph.add_edge(
                                format!("type:{name}"),
                                format!("type:{parent}"),
                                EdgeKind::Implements,
                            );
                        }
                    }
                }
            }
        }
        if trimmed.starts_with("using ") {
            let imported = trimmed
                .trim_start_matches("using ")
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

fn parse_sql(graph: &mut Graph, path: &str, content: &str) {
    let mut current: Option<String> = None;
    for line in content.lines() {
        let trimmed = line.trim().trim_end_matches(';');
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with("create table") {
            let name = trimmed
                .split_whitespace()
                .nth(2)
                .unwrap_or_default()
                .trim_matches(['"', '`', '[', ']', '(']);
            if !name.is_empty() {
                let node_id = format!("type:{name}");
                current = Some(node_id.clone());
                graph.upsert_node(Node {
                    id: node_id,
                    name: name.to_string(),
                    kind: NodeKind::Entity,
                    path: path.to_string(),
                    layer: Some("data".to_string()),
                });
            }
        } else if let Some(source) = &current {
            if lower.contains("references ") {
                let target = lower
                    .split("references ")
                    .nth(1)
                    .and_then(|value| value.split([' ', '(']).next())
                    .unwrap_or_default();
                if !target.is_empty() {
                    graph.add_edge(
                        source.clone(),
                        format!("type:{target}"),
                        EdgeKind::RelatesTo,
                    );
                }
            }
            if trimmed == ")" {
                current = None;
            }
        }
    }
}

fn parse_manifest(graph: &mut Graph, path: &str, content: &str) {
    let interesting = [
        "package.json",
        "Cargo.toml",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "apex.rules.yaml",
        "apex.workspace.yaml",
    ];
    let file_name = path.rsplit('/').next().unwrap_or(path);
    let is_interesting = interesting.contains(&file_name)
        || content.contains("\"dependencies\"")
        || content.contains("[dependencies]");
    if is_interesting {
        let node_id = format!("type:manifest:{file_name}");
        graph.upsert_node(Node {
            id: node_id,
            name: file_name.to_string(),
            kind: NodeKind::File,
            path: path.to_string(),
            layer: Some("config".to_string()),
        });
    }
}

fn insert_file_node(graph: &mut Graph, path: &str) -> String {
    let file_id = format!("file:{path}");
    graph.upsert_node(Node {
        id: file_id.clone(),
        name: path.to_string(),
        kind: NodeKind::File,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    file_id
}

fn insert_type_node(graph: &mut Graph, path: &str, file_id: &str, name: &str, kind: NodeKind) {
    let clean = name.trim_matches(['{', '}', '(', ')', ':', ',', ';']);
    if clean.is_empty() {
        return;
    }
    let node_id = format!("type:{clean}");
    graph.upsert_node(Node {
        id: node_id.clone(),
        name: clean.to_string(),
        kind,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    graph.add_edge(file_id.to_string(), node_id, EdgeKind::Contains);
}

fn go_type_name(line: &str, prefix: &str, marker: &str) -> Option<String> {
    if !line.starts_with(prefix) || !line.contains(marker) {
        return None;
    }
    line.trim_start_matches(prefix)
        .split_whitespace()
        .next()
        .map(ToOwned::to_owned)
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
    let lower = path.to_ascii_lowercase();
    if lower.contains("/api/") || lower.starts_with("api/") || lower.contains("controller") {
        Some("api".to_string())
    } else if lower.contains("/infrastructure/")
        || lower.starts_with("infrastructure/")
        || lower.contains("/infra/")
    {
        Some("infrastructure".to_string())
    } else if lower.contains("repository") || lower.contains("/data/") || lower.contains("/db/") {
        Some("data".to_string())
    } else if lower.contains("service") || lower.contains("/domain/") {
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

fn yaml_value(value: &str) -> String {
    value.trim().trim_matches(['"', '\'', ' ']).to_string()
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

    #[test]
    fn test_supported_languages_include_go_rust_kotlin_and_csharp() {
        let names: Vec<_> = supported_languages()
            .into_iter()
            .map(|language| language.name)
            .collect();

        assert!(names.contains(&"Go"));
        assert!(names.contains(&"Rust"));
        assert!(names.contains(&"Kotlin"));
        assert!(names.contains(&"C#"));
    }

    #[test]
    fn test_load_rules_parses_forbidden_import_rule() {
        let root = std::env::temp_dir().join(format!("apex-rules-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp dir");
        let rules_path = root.join("apex.rules.yaml");
        fs::write(
            &rules_path,
            "version: 1\nrules:\n  - id: CUSTOM-001\n    type: forbidden_import\n    from: service\n    to: data\n    enabled: true\n",
        )
        .expect("write rules");

        let rules = load_rules(&rules_path).expect("load rules");

        assert_eq!(rules[0].id, "CUSTOM-001");
        assert_eq!(rules[0].from.as_deref(), Some("service"));
        let _ = fs::remove_dir_all(root);
    }
}
