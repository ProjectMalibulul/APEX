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
        let diagram_ids = self.diagram_node_ids();
        for id in &diagram_ids {
            if let Some(node) = self.nodes.get(id) {
                out.push_str(&format!("  class {}\n", sanitize_mermaid(&node.name)));
            }
        }
        for edge in &self.edges {
            if matches!(edge.kind, EdgeKind::Contains)
                || !diagram_ids.contains(&edge.from)
                || !diagram_ids.contains(&edge.to)
            {
                continue;
            }
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
        let layout = diagram_layout(self);
        let width = 260usize.max(layout.len() * 180 + 40);
        let height = 240usize.max(layout.values().map(|(_, y)| *y).max().unwrap_or(0) + 100);
        let mut out = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-labelledby=\"title\" viewBox=\"0 0 {} {}\"><title id=\"title\">Apex architecture diagram</title>",
            width, height
        );
        for edge in &self.edges {
            if matches!(edge.kind, EdgeKind::Contains) {
                continue;
            }
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

    fn diagram_node_ids(&self) -> BTreeSet<String> {
        self.nodes
            .iter()
            .filter(|(_, node)| matches!(node.kind, NodeKind::Type | NodeKind::Entity))
            .map(|(id, _)| id.clone())
            .collect()
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

fn diagram_layout(graph: &Graph) -> BTreeMap<String, (usize, usize)> {
    let diagram_ids = graph.diagram_node_ids();
    let mut remaining = diagram_ids.clone();
    let mut components: Vec<Vec<String>> = Vec::new();
    while let Some(seed) = remaining.iter().next().cloned() {
        let mut component = Vec::new();
        let mut queue = VecDeque::from([seed.clone()]);
        remaining.remove(&seed);
        while let Some(current) = queue.pop_front() {
            component.push(current.clone());
            for edge in graph
                .edges
                .iter()
                .filter(|edge| !matches!(edge.kind, EdgeKind::Contains))
            {
                let next = if edge.from == current && diagram_ids.contains(&edge.to) {
                    Some(edge.to.clone())
                } else if edge.to == current && diagram_ids.contains(&edge.from) {
                    Some(edge.from.clone())
                } else {
                    None
                };
                if let Some(next) = next {
                    if remaining.remove(&next) {
                        queue.push_back(next);
                    }
                }
            }
        }
        component.sort();
        components.push(component);
    }
    components.sort_by(|left, right| left.first().cmp(&right.first()));
    let mut layout = BTreeMap::new();
    let mut y_offset = 40usize;
    for component in components {
        for (index, id) in component.iter().enumerate() {
            layout.insert(
                id.clone(),
                (40 + (index % 4) * 180, y_offset + (index / 4) * 110),
            );
        }
        y_offset += ((component.len().saturating_sub(1) / 4) + 1) * 110 + 70;
    }
    layout
}

/// Parses a repository into a graph using lightweight language recognizers.
pub fn parse_repository(root: &Path) -> io::Result<Graph> {
    let mut graph = Graph::new();
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files.sort();
    for path in &files {
        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        let Some(content) = read_source_text(path)? else {
            continue;
        };
        let relative = relative_path(root, path);
        match ext.as_str() {
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
        if is_parseable_path(root) {
            out.push(root.to_path_buf());
        }
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
        } else if is_parseable_path(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_parseable_path(path: &Path) -> bool {
    let file_name = path.file_name().and_then(|value| value.to_str());
    if matches!(file_name, Some("go.mod" | "pom.xml" | "build.gradle")) {
        return true;
    }
    matches!(
        path.extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase())
            .as_deref(),
        Some(
            "ts" | "tsx"
                | "js"
                | "jsx"
                | "py"
                | "java"
                | "go"
                | "rs"
                | "kt"
                | "kts"
                | "cs"
                | "prisma"
                | "sql"
                | "json"
                | "toml"
                | "yaml"
                | "yml"
        )
    )
}

fn read_source_text(path: &Path) -> io::Result<Option<String>> {
    let bytes = fs::read(path)?;
    if bytes.iter().take(4096).any(|byte| *byte == 0) {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&bytes).into_owned()))
}

fn parse_typescript(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut local_types = Vec::new();
    let mut imports = Vec::new();
    for line in content.lines() {
        if let Some(name) =
            after_keyword(line, "class").or_else(|| after_keyword(line, "interface"))
        {
            let node_id = insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
            local_types.push(node_id.clone());
            if let Some(parent) = after_keyword(line, "extends") {
                graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
            }
            if let Some(interface) = after_keyword(line, "implements") {
                graph.add_edge(node_id, format!("type:{interface}"), EdgeKind::Implements);
            }
        }
        if line.trim_start().starts_with("import ") {
            for imported in imported_symbols(line) {
                imports.push((imported, layer_for_import(line)));
            }
        }
    }
    add_import_edges(graph, path, &local_types, imports);
}

fn parse_python(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut current_class: Option<String> = None;
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
            let node_id = insert_type_node(
                graph,
                path,
                &file_id,
                &name,
                if content.contains("models.Model") {
                    NodeKind::Entity
                } else {
                    NodeKind::Type
                },
            );
            current_class = Some(node_id.clone());
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
                let source = current_class.clone().unwrap_or_else(|| file_id.clone());
                graph.add_edge(source, format!("type:{target}"), EdgeKind::RelatesTo);
            }
        }
    }
}

fn parse_java(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut local_types = Vec::new();
    let mut imports = Vec::new();
    for line in content.lines() {
        if let Some(name) =
            after_keyword(line, "class").or_else(|| after_keyword(line, "interface"))
        {
            let entity = content.contains("@Entity");
            let node_id = insert_type_node(
                graph,
                path,
                &file_id,
                &name,
                if entity {
                    NodeKind::Entity
                } else {
                    NodeKind::Type
                },
            );
            local_types.push(node_id.clone());
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
                imports.push((imported.to_string(), layer_for_import(line)));
            }
        }
    }
    add_import_edges(graph, path, &local_types, imports);
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
    let mut current_type: Option<String> = None;
    let mut brace_depth = 0usize;
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
            let node_id = insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
            current_type = Some(node_id);
            brace_depth = count_char(trimmed, '{').saturating_sub(count_char(trimmed, '}'));
            for (_, target) in qualified_type_refs(trimmed) {
                if let Some(source) = &current_type {
                    add_semantic_reference(graph, source, &target, path, EdgeKind::Imports, None);
                }
            }
            continue;
        }
        if let Some(source) = &current_type {
            for (qualifier, target) in qualified_type_refs(trimmed) {
                let target_path = format!("{qualifier}.{target}");
                add_semantic_reference(
                    graph,
                    source,
                    &target,
                    &target_path,
                    EdgeKind::Imports,
                    layer_for_reference(&qualifier),
                );
            }
            brace_depth += count_char(trimmed, '{');
            brace_depth = brace_depth.saturating_sub(count_char(trimmed, '}'));
            if brace_depth == 0 && trimmed.contains('}') {
                current_type = None;
            }
        }
    }
}

fn parse_rust(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut local_types = Vec::new();
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["struct", "enum", "trait"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                let node_id = insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
                local_types.push(node_id);
            }
        }
        if let Some(rest) = trimmed.strip_prefix("impl ") {
            if let Some((trait_name, type_name)) = rest.split_once(" for ") {
                let trait_name = trait_name
                    .trim()
                    .split('<')
                    .next()
                    .unwrap_or(trait_name)
                    .trim();
                let type_name = type_name
                    .trim()
                    .split(['<', '{', ' '])
                    .next()
                    .unwrap_or_default();
                if !trait_name.is_empty() && !type_name.is_empty() {
                    graph.add_edge(
                        format!("type:{type_name}"),
                        format!("type:{trait_name}"),
                        EdgeKind::Implements,
                    );
                }
            } else if let Some(name) = after_keyword(trimmed, "impl") {
                let clean = name.split('<').next().unwrap_or(&name).trim();
                if !clean.is_empty() && clean != "for" {
                    let node_id = insert_type_node(graph, path, &file_id, clean, NodeKind::Type);
                    local_types.push(node_id);
                }
            }
        }
        if trimmed.starts_with("use ") {
            let imported = trimmed
                .trim_start_matches("use ")
                .trim_end_matches(';')
                .split("::")
                .last()
                .unwrap_or_default();
            if !imported.is_empty()
                && imported != "crate"
                && imported != "self"
                && imported != "super"
            {
                imports.push((imported.to_string(), None));
            }
        }
    }
    add_import_edges(graph, path, &local_types, imports);
}

fn parse_kotlin(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut local_types = Vec::new();
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["class", "interface", "object", "data"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                let node_id = insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
                local_types.push(node_id);
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
                imports.push((imported.to_string(), layer_for_import(line)));
            }
        }
    }
    add_import_edges(graph, path, &local_types, imports);
}

fn parse_csharp(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut local_types = Vec::new();
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        for keyword in ["class", "interface", "struct", "record"] {
            if let Some(name) = after_keyword(trimmed, keyword) {
                let node_id = insert_type_node(graph, path, &file_id, &name, NodeKind::Type);
                local_types.push(node_id);
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
                imports.push((imported.to_string(), layer_for_import(line)));
            }
        }
    }
    add_import_edges(graph, path, &local_types, imports);
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

fn insert_type_node(
    graph: &mut Graph,
    path: &str,
    file_id: &str,
    name: &str,
    kind: NodeKind,
) -> String {
    let clean = name.trim_matches(['{', '}', '(', ')', ':', ',', ';']);
    if clean.is_empty() {
        return String::new();
    }
    let node_id = format!("type:{clean}");
    graph.upsert_node(Node {
        id: node_id.clone(),
        name: clean.to_string(),
        kind,
        path: path.to_string(),
        layer: layer_for_path(path),
    });
    graph.add_edge(file_id.to_string(), node_id.clone(), EdgeKind::Contains);
    node_id
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

fn add_import_edges(
    graph: &mut Graph,
    _path: &str,
    local_types: &[String],
    imports: Vec<(String, Option<String>)>,
) {
    for source in local_types {
        for (target, layer) in &imports {
            add_semantic_reference(
                graph,
                source,
                target,
                target,
                EdgeKind::Imports,
                layer.clone(),
            );
        }
    }
}

fn add_semantic_reference(
    graph: &mut Graph,
    source: &str,
    target: &str,
    target_path: &str,
    kind: EdgeKind,
    layer: Option<String>,
) {
    let clean = target
        .trim_matches(['{', '}', '(', ')', ':', ',', ';', '*', '&', '[', ']'])
        .trim();
    if clean.is_empty() || clean == source.trim_start_matches("type:") || is_builtin_type(clean) {
        return;
    }
    let target_id = format!("type:{clean}");
    if !graph.nodes.contains_key(&target_id) {
        graph.upsert_node(Node {
            id: target_id.clone(),
            name: clean.to_string(),
            kind: NodeKind::Type,
            path: target_path.to_string(),
            layer,
        });
    }
    graph.add_edge(source.to_string(), target_id, kind);
}

fn qualified_type_refs(line: &str) -> Vec<(String, String)> {
    let mut refs = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        if is_ident_start(chars[index]) {
            let start = index;
            index += 1;
            while index < chars.len() && is_ident_continue(chars[index]) {
                index += 1;
            }
            if index < chars.len() && chars[index] == '.' {
                let qualifier: String = chars[start..index].iter().collect();
                index += 1;
                if index < chars.len() && is_ident_start(chars[index]) {
                    let target_start = index;
                    index += 1;
                    while index < chars.len() && is_ident_continue(chars[index]) {
                        index += 1;
                    }
                    let target: String = chars[target_start..index].iter().collect();
                    if target.chars().next().is_some_and(char::is_uppercase) {
                        refs.push((qualifier, target));
                    }
                }
            }
        } else {
            index += 1;
        }
    }
    refs
}

fn count_char(value: &str, needle: char) -> usize {
    value.chars().filter(|ch| *ch == needle).count()
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn is_builtin_type(value: &str) -> bool {
    matches!(
        value,
        "string"
            | "str"
            | "String"
            | "bool"
            | "boolean"
            | "int"
            | "i32"
            | "i64"
            | "u32"
            | "u64"
            | "usize"
            | "float"
            | "float64"
            | "double"
            | "void"
            | "None"
            | "Option"
            | "Result"
    )
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

fn layer_for_reference(qualifier: &str) -> Option<String> {
    let lower = qualifier.to_ascii_lowercase();
    if lower.contains("repo") || lower.contains("data") || lower.contains("db") {
        Some("data".to_string())
    } else if lower.contains("infra") || lower.contains("cache") {
        Some("infrastructure".to_string())
    } else if lower.contains("api") || lower.contains("controller") {
        Some("api".to_string())
    } else if lower.contains("service") || lower.contains("domain") {
        Some("service".to_string())
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

    #[test]
    fn test_go_parser_creates_symbol_edges_not_package_edges() {
        let root = std::env::temp_dir().join(format!("apex-go-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp dir");
        fs::write(
            root.join("service.go"),
            "package service\n\nimport (\n    \"example.com/acme/repository\"\n)\n\ntype UserService struct {\n    repository repository.UserRepository\n}\n\ntype UserReader interface {\n    FindUser(id string) string\n}\n\nfunc NewUserService(repository repository.UserRepository) UserService {\n    return UserService{repository: repository}\n}\n",
        )
        .expect("write go fixture");

        let graph = parse_repository(&root).expect("parse go fixture");

        assert!(graph.nodes.contains_key("type:UserService"));
        assert!(graph.nodes.contains_key("type:UserRepository"));
        assert!(graph.nodes.contains_key("type:UserReader"));
        assert!(!graph.nodes.contains_key("type:repository"));
        assert!(!graph.nodes.contains_key("type:NewUserService"));
        assert!(graph.edges.contains(&Edge {
            from: "type:UserService".to_string(),
            to: "type:UserRepository".to_string(),
            kind: EdgeKind::Imports,
        }));
        assert!(!graph
            .edges
            .iter()
            .any(|edge| edge.from.starts_with("file:") && matches!(edge.kind, EdgeKind::Imports)));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_parse_repository_ignores_binary_non_source_files() {
        let root = std::env::temp_dir().join(format!("apex-binary-skip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("lib")).expect("create temp dirs");
        fs::write(
            root.join("UserService.java"),
            "package app;\n\nclass UserService {}\n",
        )
        .expect("write java fixture");
        fs::write(root.join("archive.zip"), [0x50, 0x4b, 0x99, 0x00]).expect("write zip");
        fs::write(
            root.join("lib").join("dependency.jar"),
            [0xca, 0xfe, 0xba, 0xbe],
        )
        .expect("write jar");
        fs::write(root.join("diagram.png"), [0x89, b'P', b'N', b'G']).expect("write png");

        let graph = parse_repository(&root).expect("binary sidecars should not fail scan");

        assert!(graph.nodes.contains_key("type:UserService"));
        assert!(!graph.nodes.contains_key("file:archive.zip"));
        assert!(!graph.nodes.contains_key("file:lib/dependency.jar"));
        assert!(!graph.nodes.contains_key("file:diagram.png"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_parse_repository_tolerates_non_utf8_source_bytes() {
        let root = std::env::temp_dir().join(format!("apex-lossy-source-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp dir");
        fs::write(
            root.join("Legacy.java"),
            [
                b'c', b'l', b'a', b's', b's', b' ', b'L', b'e', b'g', b'a', b'c', b'y', b' ', b'{',
                b' ', 0xff, b' ', b'}',
            ],
        )
        .expect("write legacy java fixture");

        let graph = parse_repository(&root).expect("legacy source encoding should not fail scan");

        assert!(graph.nodes.contains_key("type:Legacy"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_disconnected_type_nodes_render_without_forced_edges() {
        let mut graph = Graph::new();
        graph.upsert_node(Node {
            id: "type:Alpha".into(),
            name: "Alpha".into(),
            kind: NodeKind::Type,
            path: "alpha.go".into(),
            layer: None,
        });
        graph.upsert_node(Node {
            id: "type:Beta".into(),
            name: "Beta".into(),
            kind: NodeKind::Type,
            path: "beta.go".into(),
            layer: None,
        });

        let mermaid = graph.to_mermaid();
        let svg = graph.to_svg();

        assert!(mermaid.contains("class Alpha"));
        assert!(mermaid.contains("class Beta"));
        assert!(!mermaid.contains("-->"));
        assert!(svg.contains("Alpha"));
        assert!(svg.contains("Beta"));
        assert!(!svg.contains("<line"));
    }
}
