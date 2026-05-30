use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A member declared inside a type (field or method).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Member {
    /// Display name of the member.
    pub name: String,
    /// Member category: `field` or `method`.
    pub kind: MemberKind,
}

/// Classification for a `Member`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MemberKind {
    /// Method or function declared inside a type.
    Method,
    /// Field, property, or attribute declared inside a type.
    Field,
}

impl MemberKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Method => "method",
            Self::Field => "field",
        }
    }
}

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
    /// Members (fields/methods) discovered for each type node, keyed by node id.
    pub members: BTreeMap<String, Vec<Member>>,
}

/// Metrics derived from a [`Graph`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphMetrics {
    /// Total number of nodes.
    pub node_count: usize,
    /// Total number of edges.
    pub edge_count: usize,
    /// Number of weakly-connected components within the diagram subgraph.
    pub component_count: usize,
    /// Detected import cycles, each as the participating node ids.
    pub cycles: Vec<Vec<String>>,
    /// Top hotspot nodes by combined fan-in + fan-out.
    pub hotspots: Vec<NodeMetric>,
    /// Fan-in count keyed by node id.
    pub fan_in: BTreeMap<String, usize>,
    /// Fan-out count keyed by node id.
    pub fan_out: BTreeMap<String, usize>,
    /// Number of nodes per detected architectural layer.
    pub layer_mix: BTreeMap<String, usize>,
    /// Disconnected type nodes (no diagram edges).
    pub orphans: Vec<String>,
    /// Edge count between layer pairs (`from -> to`).
    pub layer_edges: BTreeMap<(String, String), usize>,
}

/// Per-node metric record.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeMetric {
    /// Node id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Fan-in count (incoming edges).
    pub fan_in: usize,
    /// Fan-out count (outgoing edges).
    pub fan_out: usize,
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
        let from = from.into();
        let to = to.into();
        if let Some(name) = to.strip_prefix("type:") {
            if is_builtin_type(name) {
                return;
            }
        }
        if let Some(name) = from.strip_prefix("type:") {
            if is_builtin_type(name) {
                return;
            }
        }
        if from == to {
            return;
        }
        let edge = Edge { from, to, kind };
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
        let members = self
            .members
            .iter()
            .filter(|(id, _)| seen.contains(*id))
            .map(|(id, list)| (id.clone(), list.clone()))
            .collect();
        Self {
            nodes,
            edges,
            members,
        }
    }

    /// Records a member discovered for a type node, deduplicating by name+kind.
    pub fn add_member(&mut self, node_id: &str, member: Member) {
        if member.name.is_empty() {
            return;
        }
        let entry = self.members.entry(node_id.to_string()).or_default();
        if !entry
            .iter()
            .any(|m| m.name == member.name && m.kind == member.kind)
        {
            entry.push(member);
        }
    }

    /// Serializes graph data into deterministic JSON without external dependencies.
    pub fn to_json(&self) -> String {
        let mut nodes = String::new();
        for (index, node) in self.nodes.values().enumerate() {
            if index > 0 {
                nodes.push(',');
            }
            let members_json = self
                .members
                .get(&node.id)
                .map(|list| {
                    let parts: Vec<String> = list
                        .iter()
                        .map(|m| {
                            format!(
                                "{{\"name\":\"{}\",\"kind\":\"{}\"}}",
                                escape_json(&m.name),
                                m.kind.as_str()
                            )
                        })
                        .collect();
                    format!("[{}]", parts.join(","))
                })
                .unwrap_or_else(|| "[]".to_string());
            nodes.push_str(&format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"kind\":\"{}\",\"path\":\"{}\",\"layer\":{},\"members\":{}}}",
                escape_json(&node.id),
                escape_json(&node.name),
                node.kind.as_str(),
                escape_json(&node.path),
                node.layer
                    .as_ref()
                    .map(|layer| format!("\"{}\"", escape_json(layer)))
                    .unwrap_or_else(|| "null".to_string()),
                members_json
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
                let safe = sanitize_mermaid(&node.name);
                let members = self.members.get(id);
                if members.map(|m| !m.is_empty()).unwrap_or(false) {
                    out.push_str(&format!("  class {safe} {{\n"));
                    for member in members.unwrap() {
                        let symbol = match member.kind {
                            MemberKind::Field => "+",
                            MemberKind::Method => "+",
                        };
                        let suffix = match member.kind {
                            MemberKind::Field => "",
                            MemberKind::Method => "()",
                        };
                        out.push_str(&format!(
                            "    {symbol}{}{}\n",
                            sanitize_mermaid(&member.name),
                            suffix
                        ));
                    }
                    out.push_str("  }\n");
                } else {
                    out.push_str(&format!("  class {safe}\n"));
                }
                if matches!(node.kind, NodeKind::Entity) {
                    out.push_str(&format!("  <<entity>> {safe}\n"));
                }
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
                let label = match edge.kind {
                    EdgeKind::Extends => " : extends",
                    EdgeKind::Implements => " : implements",
                    EdgeKind::RelatesTo => " : relates",
                    EdgeKind::Imports => " : imports",
                    EdgeKind::Contains => "",
                };
                out.push_str(&format!(
                    "  {} {} {}{}\n",
                    sanitize_mermaid(&from.name),
                    arrow,
                    sanitize_mermaid(&to.name),
                    label
                ));
            }
        }
        out
    }

    /// Serializes graph data into a self-contained zoomable SVG.
    pub fn to_svg(&self) -> String {
        render_svg(self)
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
    let diagram_ids: BTreeSet<String> = graph.diagram_node_ids();
    if diagram_ids.is_empty() {
        return BTreeMap::new();
    }

    let relevant_edges: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|edge| {
            !matches!(edge.kind, EdgeKind::Contains)
                && diagram_ids.contains(&edge.from)
                && diagram_ids.contains(&edge.to)
        })
        .collect();

    // Components by undirected reachability over relevant edges.
    let mut remaining = diagram_ids.clone();
    let mut components: Vec<Vec<String>> = Vec::new();
    while let Some(seed) = remaining.iter().next().cloned() {
        let mut component = Vec::new();
        let mut queue = VecDeque::from([seed.clone()]);
        remaining.remove(&seed);
        while let Some(current) = queue.pop_front() {
            component.push(current.clone());
            for edge in &relevant_edges {
                let next = if edge.from == current {
                    Some(edge.to.clone())
                } else if edge.to == current {
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

    let col_w = 200usize;
    let row_h = 130usize;
    let pad_x = 40usize;
    let pad_y = 40usize;

    let mut layout = BTreeMap::new();
    let mut y_offset = pad_y;

    for component in components {
        let component_set: BTreeSet<&String> = component.iter().collect();
        // Compute longest-path layering ranks (depth from sources) over directed
        // relevant edges within this component. Edges go: source -> target.
        let mut rank: BTreeMap<String, usize> =
            component.iter().map(|id| (id.clone(), 0usize)).collect();

        // Iterate until fixed point; bound iterations to component size to avoid
        // long cycles eating CPU.
        let max_iter = component.len() + 4;
        for _ in 0..max_iter {
            let mut changed = false;
            for edge in &relevant_edges {
                if !component_set.contains(&edge.from) || !component_set.contains(&edge.to) {
                    continue;
                }
                let from_rank = *rank.get(&edge.from).unwrap_or(&0);
                let entry = rank.entry(edge.to.clone()).or_insert(0);
                if *entry < from_rank + 1 {
                    *entry = from_rank + 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        // Group nodes by rank (deterministic ordering).
        let mut by_rank: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for id in &component {
            let r = *rank.get(id).unwrap_or(&0);
            by_rank.entry(r).or_default().push(id.clone());
        }
        for ids in by_rank.values_mut() {
            ids.sort();
        }

        let max_per_row = 6usize;
        let mut local_rows: Vec<Vec<String>> = Vec::new();
        for (_, ids) in by_rank {
            for chunk in ids.chunks(max_per_row) {
                local_rows.push(chunk.to_vec());
            }
        }
        if local_rows.is_empty() {
            local_rows.push(component.clone());
        }

        let max_cols = local_rows.iter().map(|r| r.len()).max().unwrap_or(1);
        for (row_idx, row) in local_rows.iter().enumerate() {
            // Center rows that are narrower than max_cols.
            let row_offset = (max_cols.saturating_sub(row.len())) * col_w / 2;
            for (col, id) in row.iter().enumerate() {
                layout.insert(
                    id.clone(),
                    (pad_x + row_offset + col * col_w, y_offset + row_idx * row_h),
                );
            }
        }
        y_offset += local_rows.len() * row_h + 60;
    }

    layout
}

const NODE_W: usize = 168;
const NODE_H_BASE: usize = 56;
const NODE_H_LINE: usize = 16;

fn render_svg(graph: &Graph) -> String {
    let layout = diagram_layout(graph);
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for (x, y) in layout.values() {
        max_x = max_x.max(*x + NODE_W);
        max_y = max_y.max(*y + NODE_H_BASE + NODE_H_LINE * 4);
    }
    let width = 320usize.max(max_x + 40);
    let height = 200usize.max(max_y + 40);

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-labelledby=\"apex-title\" \
         viewBox=\"0 0 {width} {height}\" preserveAspectRatio=\"xMidYMid meet\" \
         width=\"100%\" height=\"100%\">"
    ));
    out.push_str("<title id=\"apex-title\">Apex architecture diagram</title>");
    out.push_str("<defs>");
    for (id, color) in [
        ("apex-arrow-imports", "#637083"),
        ("apex-arrow-extends", "#1f6feb"),
        ("apex-arrow-implements", "#8957e5"),
        ("apex-arrow-relates", "#bf3989"),
    ] {
        out.push_str(&format!(
            "<marker id=\"{id}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" \
             markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"{color}\"/></marker>"
        ));
    }
    out.push_str("</defs>");
    out.push_str("<g class=\"apex-graph-root\">");

    // Edges first, so nodes draw on top.
    for edge in &graph.edges {
        if matches!(edge.kind, EdgeKind::Contains) {
            continue;
        }
        let (Some((x1, y1)), Some((x2, y2))) = (layout.get(&edge.from), layout.get(&edge.to))
        else {
            continue;
        };
        let h_from = node_height(graph, &edge.from);
        let h_to = node_height(graph, &edge.to);
        let cx1 = x1 + NODE_W / 2;
        let cy1 = y1 + h_from / 2;
        let cx2 = x2 + NODE_W / 2;
        let cy2 = y2 + h_to / 2;
        let (color, marker, dash) = match edge.kind {
            EdgeKind::Imports => ("#637083", "apex-arrow-imports", "4 3"),
            EdgeKind::Extends => ("#1f6feb", "apex-arrow-extends", ""),
            EdgeKind::Implements => ("#8957e5", "apex-arrow-implements", "6 3"),
            EdgeKind::RelatesTo => ("#bf3989", "apex-arrow-relates", ""),
            EdgeKind::Contains => continue,
        };
        let dash_attr = if dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{dash}\"")
        };
        out.push_str(&format!(
            "<path class=\"apex-edge apex-edge-{kind}\" d=\"M{cx1},{cy1} L{cx2},{cy2}\" \
             fill=\"none\" stroke=\"{color}\" stroke-width=\"1.6\"{dash_attr} \
             marker-end=\"url(#{marker})\"/>",
            kind = edge.kind.as_str()
        ));
    }

    for node in graph.nodes.values() {
        let Some((x, y)) = layout.get(&node.id) else {
            continue;
        };
        let members = graph.members.get(&node.id).cloned().unwrap_or_default();
        let max_lines = 4usize;
        let visible: Vec<&Member> = members.iter().take(max_lines).collect();
        let h = NODE_H_BASE + visible.len() * NODE_H_LINE + if !visible.is_empty() { 8 } else { 0 };
        let (fill, stroke) = match node.kind {
            NodeKind::Entity => ("#fff7ec", "#bf8700"),
            NodeKind::Type => ("#f7fbff", "#1f6feb"),
            NodeKind::File => ("#f6f8fa", "#57606a"),
        };
        out.push_str(&format!(
            "<g class=\"apex-node apex-node-{kind}\" tabindex=\"0\" aria-label=\"{label}\">",
            kind = node.kind.as_str(),
            label = escape_xml(&node.name)
        ));
        out.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{NODE_W}\" height=\"{h}\" rx=\"8\" \
             fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"1.4\"/>"
        ));
        // Header band
        let header_h = 26;
        out.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{NODE_W}\" height=\"{header_h}\" rx=\"8\" \
             fill=\"{stroke}\" fill-opacity=\"0.12\" stroke=\"none\"/>",
        ));
        out.push_str(&format!(
            "<text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
             font-size=\"13\" font-weight=\"600\" fill=\"#1f2328\">{name}</text>",
            tx = x + NODE_W / 2,
            ty = y + 17,
            name = escape_xml(&node.name)
        ));
        let subtitle = node
            .layer
            .as_deref()
            .map(|l| format!("«{l}»"))
            .or_else(|| {
                let p = &node.path;
                if p.is_empty() {
                    None
                } else {
                    Some(short_path(p))
                }
            })
            .unwrap_or_default();
        if !subtitle.is_empty() {
            out.push_str(&format!(
                "<text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
                 font-size=\"10\" fill=\"#57606a\">{txt}</text>",
                tx = x + NODE_W / 2,
                ty = y + 38,
                txt = escape_xml(&subtitle)
            ));
        }
        // Member lines
        for (i, m) in visible.iter().enumerate() {
            let ty = y + 56 + i * NODE_H_LINE;
            let prefix = match m.kind {
                MemberKind::Field => "+",
                MemberKind::Method => "+",
            };
            let suffix = match m.kind {
                MemberKind::Field => "",
                MemberKind::Method => "()",
            };
            out.push_str(&format!(
                "<text x=\"{tx}\" y=\"{ty}\" font-family=\"ui-monospace,monospace\" \
                 font-size=\"11\" fill=\"#1f2328\">{txt}</text>",
                tx = x + 10,
                txt = escape_xml(&format!("{prefix}{}{}", m.name, suffix))
            ));
        }
        if members.len() > visible.len() {
            let ty = y + 56 + visible.len() * NODE_H_LINE;
            out.push_str(&format!(
                "<text x=\"{tx}\" y=\"{ty}\" font-family=\"system-ui,sans-serif\" \
                 font-size=\"10\" fill=\"#57606a\">{txt}</text>",
                tx = x + 10,
                txt = escape_xml(&format!("…(+{} more)", members.len() - visible.len()))
            ));
        }
        out.push_str("</g>");
    }

    out.push_str("</g></svg>");
    out
}

fn node_height(graph: &Graph, id: &str) -> usize {
    let count = graph.members.get(id).map(|m| m.len().min(4)).unwrap_or(0);
    NODE_H_BASE + count * NODE_H_LINE + if count > 0 { 8 } else { 0 }
}

fn short_path(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() <= 2 {
        path.to_string()
    } else {
        format!(".../{}", parts[parts.len() - 2..].join("/"))
    }
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
            "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" => {
                parse_typescript(&mut graph, &relative, &content)
            }
            "py" => parse_python(&mut graph, &relative, &content),
            "java" => parse_java(&mut graph, &relative, &content),
            "go" => parse_go(&mut graph, &relative, &content),
            "rs" => parse_rust(&mut graph, &relative, &content),
            "kt" | "kts" => parse_kotlin(&mut graph, &relative, &content),
            "cs" => parse_csharp(&mut graph, &relative, &content),
            "sql" => parse_sql(&mut graph, &relative, &content),
            "prisma" => parse_prisma(&mut graph, &relative, &content),
            "c" | "cc" | "cpp" | "cxx" | "h" | "hpp" | "hh" | "hxx" => {
                parse_cpp(&mut graph, &relative, &content)
            }
            "swift" => parse_swift(&mut graph, &relative, &content),
            "php" => parse_php(&mut graph, &relative, &content),
            "rb" => parse_ruby(&mut graph, &relative, &content),
            "scala" | "sc" => parse_scala(&mut graph, &relative, &content),
            "dart" => parse_dart(&mut graph, &relative, &content),
            "json" | "toml" | "yaml" | "yml" => parse_manifest(&mut graph, &relative, &content),
            _ => {
                if relative.ends_with("go.mod")
                    || relative.ends_with("pom.xml")
                    || relative.ends_with("build.gradle")
                    || relative.ends_with("Gemfile")
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
            extensions: &["ts", "tsx", "js", "jsx", "mjs", "cjs"],
            extracts: "classes, interfaces, members, imports, extends, implements",
        },
        LanguageSupport {
            name: "Python",
            extensions: &["py"],
            extracts: "classes, base classes, Django-style model relations",
        },
        LanguageSupport {
            name: "Java",
            extensions: &["java"],
            extracts: "classes, interfaces, members, Spring/JPA annotations, imports, implements",
        },
        LanguageSupport {
            name: "Go",
            extensions: &["go"],
            extracts: "structs, interfaces, fields, functions, qualified type references",
        },
        LanguageSupport {
            name: "Rust",
            extensions: &["rs"],
            extracts: "structs, enums, traits, impls, fields, use imports",
        },
        LanguageSupport {
            name: "Kotlin",
            extensions: &["kt", "kts"],
            extracts: "classes, interfaces, objects, members, imports, inheritance",
        },
        LanguageSupport {
            name: "C#",
            extensions: &["cs"],
            extracts: "classes, interfaces, structs, records, members, using imports, inheritance",
        },
        LanguageSupport {
            name: "C / C++",
            extensions: &["c", "cc", "cpp", "cxx", "h", "hpp", "hh", "hxx"],
            extracts: "classes, structs, members, includes, inheritance",
        },
        LanguageSupport {
            name: "Swift",
            extensions: &["swift"],
            extracts: "classes, structs, protocols, actors, members, imports",
        },
        LanguageSupport {
            name: "PHP",
            extensions: &["php"],
            extracts: "classes, interfaces, traits, members, use imports, inheritance",
        },
        LanguageSupport {
            name: "Ruby",
            extensions: &["rb"],
            extracts: "classes, modules, methods, requires, inheritance",
        },
        LanguageSupport {
            name: "Scala",
            extensions: &["scala", "sc"],
            extracts: "classes, traits, objects, members, imports",
        },
        LanguageSupport {
            name: "Dart",
            extensions: &["dart"],
            extracts: "classes, mixins, members, imports, extends/implements/with",
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
            extracts: "package/config files plus npm and Cargo dependency edges",
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

/// Computes architectural metrics for a parsed graph.
pub fn compute_metrics(graph: &Graph) -> GraphMetrics {
    let mut metrics = GraphMetrics {
        node_count: graph.nodes.len(),
        edge_count: graph.edges.len(),
        ..GraphMetrics::default()
    };

    let diagram_ids = graph.diagram_node_ids();
    let relevant: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|e| !matches!(e.kind, EdgeKind::Contains))
        .filter(|e| graph.nodes.contains_key(&e.from) && graph.nodes.contains_key(&e.to))
        .collect();

    for edge in &relevant {
        *metrics.fan_out.entry(edge.from.clone()).or_insert(0) += 1;
        *metrics.fan_in.entry(edge.to.clone()).or_insert(0) += 1;
        if let (Some(from), Some(to)) = (graph.nodes.get(&edge.from), graph.nodes.get(&edge.to)) {
            if let (Some(fl), Some(tl)) = (from.layer.as_ref(), to.layer.as_ref()) {
                *metrics
                    .layer_edges
                    .entry((fl.clone(), tl.clone()))
                    .or_insert(0) += 1;
            }
        }
    }

    for (id, node) in &graph.nodes {
        if let Some(layer) = &node.layer {
            *metrics.layer_mix.entry(layer.clone()).or_insert(0) += 1;
        }
        if matches!(node.kind, NodeKind::Type | NodeKind::Entity) {
            let touched = relevant.iter().any(|e| e.from == *id || e.to == *id);
            if !touched {
                metrics.orphans.push(id.clone());
            }
        }
    }
    metrics.orphans.sort();

    // Components (undirected over diagram nodes only).
    let mut remaining = diagram_ids.clone();
    let mut components = 0usize;
    while let Some(seed) = remaining.iter().next().cloned() {
        components += 1;
        let mut queue = VecDeque::from([seed.clone()]);
        remaining.remove(&seed);
        while let Some(current) = queue.pop_front() {
            for e in &relevant {
                let next = if e.from == current && diagram_ids.contains(&e.to) {
                    Some(e.to.clone())
                } else if e.to == current && diagram_ids.contains(&e.from) {
                    Some(e.from.clone())
                } else {
                    None
                };
                if let Some(n) = next {
                    if remaining.remove(&n) {
                        queue.push_back(n);
                    }
                }
            }
        }
    }
    metrics.component_count = components;

    // Cycles via DFS (Tarjan-lite limited to import edges).
    let import_edges: Vec<&Edge> = relevant
        .iter()
        .copied()
        .filter(|e| matches!(e.kind, EdgeKind::Imports))
        .collect();
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for e in &import_edges {
        adj.entry(e.from.clone()).or_default().push(e.to.clone());
    }
    let mut cycles: Vec<Vec<String>> = Vec::new();
    let mut on_stack: BTreeSet<String> = BTreeSet::new();
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut stack: Vec<String> = Vec::new();
    let nodes_sorted: Vec<String> = adj.keys().cloned().collect();
    for start in nodes_sorted {
        if visited.contains(&start) {
            continue;
        }
        cycle_dfs(
            &start,
            &adj,
            &mut visited,
            &mut on_stack,
            &mut stack,
            &mut cycles,
        );
    }
    metrics.cycles = cycles;

    // Hotspots: top by fan_in + fan_out.
    let mut all_ids: BTreeSet<String> = BTreeSet::new();
    all_ids.extend(metrics.fan_in.keys().cloned());
    all_ids.extend(metrics.fan_out.keys().cloned());
    let mut scored: Vec<NodeMetric> = all_ids
        .into_iter()
        .map(|id| {
            let fan_in = *metrics.fan_in.get(&id).unwrap_or(&0);
            let fan_out = *metrics.fan_out.get(&id).unwrap_or(&0);
            let name = graph
                .nodes
                .get(&id)
                .map(|n| n.name.clone())
                .unwrap_or_else(|| id.clone());
            NodeMetric {
                id,
                name,
                fan_in,
                fan_out,
            }
        })
        .collect();
    scored.sort_by(|a, b| {
        let total_a = a.fan_in + a.fan_out;
        let total_b = b.fan_in + b.fan_out;
        total_b.cmp(&total_a).then(a.name.cmp(&b.name))
    });
    scored.truncate(10);
    metrics.hotspots = scored;
    metrics
}

fn cycle_dfs(
    node: &str,
    adj: &BTreeMap<String, Vec<String>>,
    visited: &mut BTreeSet<String>,
    on_stack: &mut BTreeSet<String>,
    stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    on_stack.insert(node.to_string());
    stack.push(node.to_string());

    if let Some(neigh) = adj.get(node) {
        for next in neigh {
            if !visited.contains(next) {
                cycle_dfs(next, adj, visited, on_stack, stack, cycles);
            } else if on_stack.contains(next) {
                if let Some(pos) = stack.iter().position(|n| n == next) {
                    let cycle: Vec<String> = stack[pos..].to_vec();
                    if !cycles.iter().any(|c| same_cycle(c, &cycle)) {
                        cycles.push(cycle);
                    }
                }
            }
        }
    }
    on_stack.remove(node);
    stack.pop();
}

fn same_cycle(a: &[String], b: &[String]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let len = a.len();
    for shift in 0..len {
        if (0..len).all(|i| a[(i + shift) % len] == b[i]) {
            return true;
        }
    }
    false
}

/// Renders [`GraphMetrics`] as deterministic JSON.
pub fn metrics_to_json(metrics: &GraphMetrics) -> String {
    let cycles = metrics
        .cycles
        .iter()
        .map(|cycle| {
            let parts: Vec<String> = cycle
                .iter()
                .map(|id| format!("\"{}\"", escape_json(id)))
                .collect();
            format!("[{}]", parts.join(","))
        })
        .collect::<Vec<_>>()
        .join(",");
    let hotspots = metrics
        .hotspots
        .iter()
        .map(|m| {
            format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"fan_in\":{},\"fan_out\":{}}}",
                escape_json(&m.id),
                escape_json(&m.name),
                m.fan_in,
                m.fan_out
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let layer_mix = metrics
        .layer_mix
        .iter()
        .map(|(k, v)| format!("\"{}\":{}", escape_json(k), v))
        .collect::<Vec<_>>()
        .join(",");
    let layer_edges = metrics
        .layer_edges
        .iter()
        .map(|((f, t), v)| {
            format!(
                "{{\"from\":\"{}\",\"to\":\"{}\",\"count\":{}}}",
                escape_json(f),
                escape_json(t),
                v
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let orphans = metrics
        .orphans
        .iter()
        .map(|id| format!("\"{}\"", escape_json(id)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"node_count\":{},\"edge_count\":{},\"component_count\":{},\"cycles\":[{}],\"hotspots\":[{}],\"layer_mix\":{{{}}},\"layer_edges\":[{}],\"orphans\":[{}]}}",
        metrics.node_count,
        metrics.edge_count,
        metrics.component_count,
        cycles,
        hotspots,
        layer_mix,
        layer_edges,
        orphans
    )
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
    if matches!(
        file_name,
        Some("go.mod" | "pom.xml" | "build.gradle" | "Gemfile" | "Dockerfile")
    ) {
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
                | "mjs"
                | "cjs"
                | "py"
                | "java"
                | "go"
                | "rs"
                | "kt"
                | "kts"
                | "cs"
                | "prisma"
                | "sql"
                | "c"
                | "cc"
                | "cpp"
                | "cxx"
                | "h"
                | "hpp"
                | "hh"
                | "hxx"
                | "swift"
                | "php"
                | "rb"
                | "scala"
                | "sc"
                | "dart"
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

/// A slice of source text identified as one type body, used for body-aware
/// reference detection so an import only produces an edge when actually used.
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct TypeSpan {
    name: String,
    node_id: String,
    body: String,
    header: String,
}

/// Extracts brace-delimited type bodies for a parsed file. `keywords` are the
/// declaration starters (e.g. `class`, `interface`, `struct`).
fn extract_brace_type_spans(content: &str, keywords: &[&str]) -> Vec<TypeSpan> {
    let mut spans: Vec<TypeSpan> = Vec::new();
    let mut current: Option<TypeSpan> = None;
    let mut depth: usize = 0;
    let mut entered: bool = false;

    for line in content.lines() {
        if let Some(span) = current.as_mut() {
            span.body.push_str(line);
            span.body.push('\n');
            let opens = count_char(line, '{');
            let closes = count_char(line, '}');
            if !entered && opens > 0 {
                entered = true;
            }
            depth = depth.saturating_add(opens).saturating_sub(closes);
            if entered && depth == 0 {
                let span = current.take().unwrap();
                spans.push(span);
                entered = false;
            }
            continue;
        }

        // Detect a type declaration on this line.
        let stripped = line.trim();
        if stripped.is_empty() {
            continue;
        }
        let mut detected: Option<String> = None;
        for kw in keywords {
            if let Some(name) = after_keyword(stripped, kw) {
                detected = Some(name);
                break;
            }
        }
        let Some(name) = detected else { continue };
        let opens = count_char(line, '{');
        let closes = count_char(line, '}');
        let mut span = TypeSpan {
            name: name.clone(),
            node_id: format!("type:{name}"),
            body: format!("{line}\n"),
            header: line.to_string(),
        };
        if opens == 0 {
            // Header without `{` yet (e.g. K&R style); enter span and wait.
            current = Some(span);
            depth = 0;
            entered = false;
        } else {
            depth = opens.saturating_sub(closes);
            if depth == 0 {
                spans.push(span);
            } else {
                span.body.clear();
                span.body.push_str(line);
                span.body.push('\n');
                current = Some(span);
                entered = true;
            }
        }
    }
    if let Some(span) = current {
        spans.push(span);
    }
    spans
}

/// Returns true if `body` contains `name` as a whole identifier token.
fn body_mentions(body: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let bytes = body.as_bytes();
    let target = name.as_bytes();
    let mut i = 0;
    while i + target.len() <= bytes.len() {
        if &bytes[i..i + target.len()] == target {
            let before = if i == 0 {
                None
            } else {
                Some(bytes[i - 1] as char)
            };
            let after = bytes.get(i + target.len()).map(|b| *b as char);
            let boundary_before = before.map(|c| !is_ident_continue(c)).unwrap_or(true);
            let boundary_after = after.map(|c| !is_ident_continue(c)).unwrap_or(true);
            if boundary_before && boundary_after {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Extracts simple member names from a brace-language type body.
fn extract_curly_members(body: &str, language: BraceLanguage) -> Vec<Member> {
    let mut members: Vec<Member> = Vec::new();
    for line in body.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('}')
            || trimmed.starts_with('{')
            || trimmed.starts_with('@')
            || trimmed.starts_with('#')
        {
            continue;
        }
        // Skip nested type declarations.
        if trimmed.starts_with("class ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("enum ")
        {
            continue;
        }
        if let Some(name) = parse_member(trimmed, language) {
            members.push(name);
        }
    }
    // Keep insertion order, dedup by (name, kind).
    let mut seen: BTreeSet<(String, MemberKind)> = BTreeSet::new();
    members.retain(|m| seen.insert((m.name.clone(), m.kind.clone())));
    members
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum BraceLanguage {
    Typescript,
    Java,
    Kotlin,
    Csharp,
    Rust,
    Go,
    Cpp,
    Swift,
    Php,
    Scala,
    Dart,
}

fn parse_member(trimmed: &str, language: BraceLanguage) -> Option<Member> {
    let line = trimmed.trim_end_matches([';', ',']);
    // Method heuristic: contains a `(` that's part of a signature, not after `=`.
    let paren = line.find('(');
    let assign = line.find('=');
    let is_method = match (paren, assign) {
        (Some(p), Some(a)) => p < a,
        (Some(_), None) => true,
        _ => false,
    };
    if is_method {
        let before = &line[..paren.unwrap()];
        let name = before
            .split_whitespace()
            .last()?
            .trim_start_matches(['*', '&']);
        let cleaned = name.trim_matches(|c: char| !is_ident_continue(c) && c != '_');
        if cleaned.is_empty() {
            return None;
        }
        if matches!(language, BraceLanguage::Rust) && !line.contains("fn ") {
            return None;
        }
        Some(Member {
            name: cleaned.to_string(),
            kind: MemberKind::Method,
        })
    } else {
        // Field heuristic per language.
        let name = match language {
            BraceLanguage::Typescript
            | BraceLanguage::Swift
            | BraceLanguage::Kotlin
            | BraceLanguage::Scala
            | BraceLanguage::Dart => {
                // `name: Type` or `let name = ...` or `var name = ...`
                let rest = line
                    .trim_start_matches("public ")
                    .trim_start_matches("private ")
                    .trim_start_matches("protected ")
                    .trim_start_matches("readonly ")
                    .trim_start_matches("static ")
                    .trim_start_matches("override ")
                    .trim_start_matches("val ")
                    .trim_start_matches("var ")
                    .trim_start_matches("let ")
                    .trim_start_matches("const ")
                    .trim_start_matches("final ");
                rest.split([':', '=', ' ']).next()?.trim().to_string()
            }
            BraceLanguage::Rust => {
                // `name: Type,`
                if !line.contains(':') {
                    return None;
                }
                line.split(':')
                    .next()?
                    .split_whitespace()
                    .last()?
                    .to_string()
            }
            BraceLanguage::Go => {
                // `Name Type`
                line.split_whitespace().next()?.to_string()
            }
            BraceLanguage::Java | BraceLanguage::Csharp | BraceLanguage::Cpp => {
                // `Type name;` -> last identifier before `;` or `=`
                let head = line.split('=').next()?.trim();
                head.split_whitespace()
                    .last()?
                    .trim_matches(['*', '&'])
                    .to_string()
            }
            BraceLanguage::Php => {
                let rest = line
                    .trim_start_matches("public ")
                    .trim_start_matches("private ")
                    .trim_start_matches("protected ")
                    .trim_start_matches("static ");
                rest.split_whitespace()
                    .find(|t| t.starts_with('$'))
                    .map(|s| s.trim_start_matches('$').to_string())
                    .unwrap_or_default()
            }
        };
        let cleaned: String = name.chars().take_while(|c| is_ident_continue(*c)).collect();
        if cleaned.is_empty() {
            return None;
        }
        Some(Member {
            name: cleaned,
            kind: MemberKind::Field,
        })
    }
}

fn parse_typescript(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "interface"]);
    let imports = collect_ts_imports(content);
    for span in &spans {
        let kind = NodeKind::Type;
        let node_id = insert_type_node(graph, path, &file_id, &span.name, kind);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent) = after_keyword(&span.header, "extends") {
            graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
        }
        for interface in after_keyword_all(&span.header, "implements") {
            graph.add_edge(
                node_id.clone(),
                format!("type:{interface}"),
                EdgeKind::Implements,
            );
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Typescript) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
    if spans.is_empty() {
        // Fall back to a file-level edge so imports are still surfaced when no
        // class container exists (functional/module style files).
        for (target, layer) in imports {
            add_semantic_reference(graph, &file_id, &target, &target, EdgeKind::Imports, layer);
        }
    }
}

fn collect_ts_imports(content: &str) -> Vec<(String, Option<String>)> {
    let mut imports = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("import ") {
            for imported in imported_symbols(line) {
                imports.push((imported, layer_for_import(line)));
            }
        }
    }
    imports
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
            let bases = rest
                .split('(')
                .nth(1)
                .and_then(|value| value.split(')').next())
                .unwrap_or("");
            let is_entity = bases.contains("models.Model");
            let node_id = insert_type_node(
                graph,
                path,
                &file_id,
                &name,
                if is_entity {
                    NodeKind::Entity
                } else {
                    NodeKind::Type
                },
            );
            current_class = Some(node_id.clone());
            if !bases.trim().is_empty() {
                let parent = bases.trim();
                graph.add_edge(node_id, format!("type:{parent}"), EdgeKind::Extends);
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
    let spans = extract_brace_type_spans(content, &["class", "interface"]);
    let imports = collect_java_style_imports(content, "import ");
    let entity_classes = annotated_java_types(content, "@Entity");
    for span in &spans {
        let kind = if entity_classes.contains(&span.name) {
            NodeKind::Entity
        } else {
            NodeKind::Type
        };
        let node_id = insert_type_node(graph, path, &file_id, &span.name, kind);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent) = after_keyword(&span.header, "extends") {
            graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
        }
        for interface in after_keyword_all(&span.header, "implements") {
            graph.add_edge(
                node_id.clone(),
                format!("type:{interface}"),
                EdgeKind::Implements,
            );
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Java) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn collect_java_style_imports(content: &str, prefix: &str) -> Vec<(String, Option<String>)> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with(prefix) {
            continue;
        }
        let imported = trimmed
            .trim_start_matches(prefix)
            .trim_end_matches(';')
            .rsplit('.')
            .next()
            .unwrap_or_default();
        if !imported.is_empty() && imported != "*" {
            imports.push((imported.to_string(), layer_for_import(line)));
        }
    }
    imports
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
    let spans = extract_brace_type_spans(content, &["struct", "enum", "trait"]);
    let imports = collect_rust_imports(content);

    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Rust) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
    // Detect `impl Trait for Type` outside of spans.
    for line in content.lines() {
        let trimmed = line.trim();
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
            }
        }
    }
}

fn collect_rust_imports(content: &str) -> Vec<(String, Option<String>)> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") {
            continue;
        }
        let body = trimmed.trim_start_matches("use ").trim_end_matches(';');
        // Handle `use a::b::{C, D as E};`
        if let Some((prefix, group)) = body.split_once('{') {
            let group = group.trim_end_matches('}');
            let _ = prefix;
            for piece in group.split(',') {
                let piece = piece.trim();
                let name = piece
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .rsplit("::")
                    .next()
                    .unwrap_or("");
                let final_name = name.split_whitespace().next().unwrap_or(name);
                if !final_name.is_empty() && final_name != "self" && final_name != "*" {
                    imports.push((final_name.to_string(), None));
                }
            }
        } else {
            let imported = body.rsplit("::").next().unwrap_or_default();
            if !imported.is_empty()
                && imported != "crate"
                && imported != "self"
                && imported != "super"
                && imported != "*"
            {
                imports.push((imported.to_string(), None));
            }
        }
    }
    imports
}

fn parse_kotlin(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "interface", "object"]);
    let imports = collect_java_style_imports(content, "import ");
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(supertypes) = span.header.split(':').nth(1) {
            let supertypes = supertypes.split('{').next().unwrap_or("");
            for supertype in supertypes.split(',') {
                let parent = supertype
                    .trim()
                    .split(['(', '<'])
                    .next()
                    .unwrap_or("")
                    .trim();
                if !parent.is_empty() {
                    graph.add_edge(
                        node_id.clone(),
                        format!("type:{parent}"),
                        EdgeKind::Extends,
                    );
                }
            }
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Kotlin) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn parse_csharp(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "interface", "struct", "record"]);
    let imports = collect_java_style_imports(content, "using ");
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent_list) = span.header.split(':').nth(1) {
            for parent in parent_list.split(',') {
                let parent = parent
                    .split([' ', '<', '{'])
                    .next()
                    .unwrap_or_default()
                    .trim();
                if !parent.is_empty() {
                    graph.add_edge(
                        node_id.clone(),
                        format!("type:{parent}"),
                        EdgeKind::Implements,
                    );
                }
            }
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Csharp) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
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
            let rest = trimmed["create table".len()..].trim_start();
            let rest = if rest.len() >= 13 && rest[..13].eq_ignore_ascii_case("if not exists") {
                rest[13..].trim_start()
            } else {
                rest
            };
            let name = rest
                .split_whitespace()
                .next()
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
            if let Some(pos) = lower.find("references ") {
                let after = &trimmed[pos + "references ".len()..];
                let target = after
                    .split([' ', '('])
                    .next()
                    .unwrap_or_default()
                    .trim_matches(['"', '`', '[', ']']);
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
        "pyproject.toml",
        "Gemfile",
        "composer.json",
    ];
    let file_name = path.rsplit('/').next().unwrap_or(path);
    let is_interesting = interesting.contains(&file_name)
        || content.contains("\"dependencies\"")
        || content.contains("[dependencies]");
    if !is_interesting {
        return;
    }
    let manifest_id = format!("type:manifest:{file_name}");
    graph.upsert_node(Node {
        id: manifest_id.clone(),
        name: file_name.to_string(),
        kind: NodeKind::File,
        path: path.to_string(),
        layer: Some("config".to_string()),
    });
    if file_name == "package.json" {
        for dep in extract_package_json_deps(content) {
            let dep_id = format!("type:pkg:{dep}");
            graph.upsert_node(Node {
                id: dep_id.clone(),
                name: dep.clone(),
                kind: NodeKind::File,
                path: format!("(npm) {dep}"),
                layer: Some("dependency".to_string()),
            });
            graph.add_edge(manifest_id.clone(), dep_id, EdgeKind::Imports);
        }
    } else if file_name == "Cargo.toml" {
        for dep in extract_cargo_deps(content) {
            let dep_id = format!("type:pkg:{dep}");
            graph.upsert_node(Node {
                id: dep_id.clone(),
                name: dep.clone(),
                kind: NodeKind::File,
                path: format!("(cargo) {dep}"),
                layer: Some("dependency".to_string()),
            });
            graph.add_edge(manifest_id.clone(), dep_id, EdgeKind::Imports);
        }
    }
}

fn extract_package_json_deps(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let sections = [
        "\"dependencies\"",
        "\"devDependencies\"",
        "\"peerDependencies\"",
    ];
    for section in sections {
        if let Some(start) = content.find(section) {
            if let Some(open) = content[start..].find('{') {
                let abs_open = start + open;
                if let Some(close) = content[abs_open..].find('}') {
                    let block = &content[abs_open + 1..abs_open + close];
                    for line in block.lines() {
                        if let Some(name) = line.trim().strip_prefix('"') {
                            if let Some(end) = name.find('"') {
                                let n = &name[..end];
                                if !n.is_empty() {
                                    out.push(n.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn extract_cargo_deps(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_deps = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_deps = matches!(
                trimmed,
                "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
            );
            continue;
        }
        if !in_deps || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(eq) = trimmed.find('=') {
            let name = trimmed[..eq].trim();
            if !name.is_empty() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn parse_cpp(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "struct"]);
    let imports = collect_cpp_imports(content);
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent_list) = span.header.split(':').nth(1) {
            for parent in parent_list.split(',') {
                let cleaned = parent
                    .trim()
                    .trim_start_matches("public ")
                    .trim_start_matches("private ")
                    .trim_start_matches("protected ")
                    .trim_start_matches("virtual ")
                    .split([' ', '<', '{'])
                    .next()
                    .unwrap_or_default()
                    .trim();
                if !cleaned.is_empty() {
                    graph.add_edge(
                        node_id.clone(),
                        format!("type:{cleaned}"),
                        EdgeKind::Extends,
                    );
                }
            }
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Cpp) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn collect_cpp_imports(content: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("#include") {
            let r = rest
                .trim()
                .trim_start_matches(['<', '"'])
                .trim_end_matches(['>', '"']);
            let stem = r
                .rsplit('/')
                .next()
                .unwrap_or(r)
                .trim_end_matches(".h")
                .trim_end_matches(".hpp");
            if !stem.is_empty() {
                out.push((stem.to_string(), None));
            }
        }
        if let Some(rest) = trimmed.strip_prefix("using ") {
            let n = rest
                .trim_end_matches(';')
                .rsplit("::")
                .next()
                .unwrap_or_default();
            if !n.is_empty() && n != "namespace" {
                out.push((n.to_string(), None));
            }
        }
    }
    out
}

fn parse_swift(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans =
        extract_brace_type_spans(content, &["class", "struct", "protocol", "enum", "actor"]);
    let imports = collect_simple_imports(content, "import ");
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent_list) = span.header.split(':').nth(1) {
            for parent in parent_list.split(',') {
                let cleaned = parent
                    .trim()
                    .split([' ', '<', '{'])
                    .next()
                    .unwrap_or_default()
                    .trim();
                if !cleaned.is_empty() {
                    graph.add_edge(
                        node_id.clone(),
                        format!("type:{cleaned}"),
                        EdgeKind::Implements,
                    );
                }
            }
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Swift) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn parse_php(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "interface", "trait"]);
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let n = rest
                .trim_end_matches(';')
                .split(" as ")
                .next()
                .unwrap_or_default()
                .rsplit('\\')
                .next()
                .unwrap_or_default();
            if !n.is_empty() {
                imports.push((n.to_string(), None));
            }
        }
    }
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent) = after_keyword(&span.header, "extends") {
            graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
        }
        if let Some(parent) = after_keyword(&span.header, "implements") {
            graph.add_edge(
                node_id.clone(),
                format!("type:{parent}"),
                EdgeKind::Implements,
            );
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Php) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn parse_ruby(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let mut current: Option<String> = None;
    let mut depth: usize = 0;
    let mut requires: Vec<String> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("require ") {
            let r = rest.trim().trim_matches(['"', '\'', ' ']);
            if let Some(name) = r.rsplit('/').next() {
                if !name.is_empty() {
                    requires.push(snake_to_camel(name));
                }
            }
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("class ")
            .or_else(|| trimmed.strip_prefix("module "))
        {
            let name = rest
                .split(['<', ' ', '\t'])
                .next()
                .unwrap_or_default()
                .trim();
            if name.is_empty() {
                continue;
            }
            let node_id = insert_type_node(graph, path, &file_id, name, NodeKind::Type);
            if let Some(parent) = rest.split('<').nth(1) {
                let parent = parent.split_whitespace().next().unwrap_or_default();
                if !parent.is_empty() {
                    graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
                }
            }
            current = Some(node_id);
            depth = 1;
            continue;
        }
        if let Some(name) = trimmed.strip_prefix("def ") {
            if let Some(node) = &current {
                let n = name
                    .split(['(', ' ', '\n'])
                    .next()
                    .unwrap_or_default()
                    .trim();
                if !n.is_empty() {
                    graph.add_member(
                        node,
                        Member {
                            name: n.to_string(),
                            kind: MemberKind::Method,
                        },
                    );
                }
            }
        }
        if trimmed == "end" && depth > 0 {
            depth -= 1;
            if depth == 0 {
                current = None;
            }
        }
        if (trimmed.starts_with("if ")
            || trimmed.starts_with("unless ")
            || trimmed.starts_with("do ")
            || trimmed.starts_with("while ")
            || trimmed.ends_with(" do"))
            && current.is_some()
            && depth > 0
        {
            depth += 1;
        }
    }
    for target in requires {
        add_semantic_reference(graph, &file_id, &target, &target, EdgeKind::Imports, None);
    }
}

/// Converts a Ruby `snake_case` require path stem to its conventional
/// `CamelCase` constant name (e.g. `user_repository` -> `UserRepository`).
fn snake_to_camel(value: &str) -> String {
    value
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn parse_scala(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "object", "trait", "case"]);
    let imports = collect_simple_imports(content, "import ");
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent) = after_keyword(&span.header, "extends") {
            graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Scala) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn parse_dart(graph: &mut Graph, path: &str, content: &str) {
    let file_id = insert_file_node(graph, path);
    let spans = extract_brace_type_spans(content, &["class", "mixin", "enum"]);
    let imports = collect_simple_imports(content, "import ");
    for span in &spans {
        let node_id = insert_type_node(graph, path, &file_id, &span.name, NodeKind::Type);
        if node_id.is_empty() {
            continue;
        }
        if let Some(parent) = after_keyword(&span.header, "extends") {
            graph.add_edge(node_id.clone(), format!("type:{parent}"), EdgeKind::Extends);
        }
        if let Some(parent) = after_keyword(&span.header, "implements") {
            graph.add_edge(
                node_id.clone(),
                format!("type:{parent}"),
                EdgeKind::Implements,
            );
        }
        if let Some(parent) = after_keyword(&span.header, "with") {
            graph.add_edge(
                node_id.clone(),
                format!("type:{parent}"),
                EdgeKind::RelatesTo,
            );
        }
        for member in extract_curly_members(&span.body, BraceLanguage::Dart) {
            graph.add_member(&node_id, member);
        }
        for (target, layer) in &imports {
            if body_mentions(&span.body, target) {
                add_semantic_reference(
                    graph,
                    &node_id,
                    target,
                    target,
                    EdgeKind::Imports,
                    layer.clone(),
                );
            }
        }
    }
}

fn collect_simple_imports(content: &str, prefix: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            let cleaned = rest.trim().trim_end_matches(';').trim_matches(['"', '\'']);
            let name = cleaned
                .rsplit(['/', '.'])
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if !name.is_empty() {
                out.push((name.to_string(), layer_for_import(line)));
            }
        }
    }
    out
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

/// Returns the set of class/interface names directly carrying `annotation`
/// (e.g. `@Entity`), checking the annotation lines immediately preceding each
/// declaration as well as annotations placed inline on the declaration line.
fn annotated_java_types(content: &str, annotation: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut pending = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let has_inline = trimmed.contains(annotation);
        if trimmed.starts_with(annotation) {
            pending = true;
            if !trimmed.contains("class ") && !trimmed.contains("interface ") {
                continue;
            }
        }
        let name = after_keyword(trimmed, "class").or_else(|| after_keyword(trimmed, "interface"));
        if let Some(name) = name {
            if pending || has_inline {
                result.insert(name);
            }
            pending = false;
        } else if !trimmed.starts_with('@') {
            pending = false;
        }
    }
    result
}

/// Returns every comma-separated type name following `keyword` (e.g. all
/// interfaces after `implements`), stopping at the body brace.
fn after_keyword_all(line: &str, keyword: &str) -> Vec<String> {
    let parts: Vec<_> = line.split_whitespace().collect();
    let Some(index) = parts.iter().position(|part| *part == keyword) else {
        return Vec::new();
    };
    let rest = parts[index + 1..].join(" ");
    let rest = rest.split('{').next().unwrap_or("");
    rest.split(',')
        .filter_map(|segment| {
            let name = segment
                .trim()
                .trim_matches([':', ';'])
                .split(['<', '(', '{'])
                .next()
                .unwrap_or("")
                .trim();
            (!name.is_empty()).then(|| name.to_string())
        })
        .collect()
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
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
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
    }

    #[test]
    fn test_body_aware_imports_avoid_blanket_edges() {
        // Two TS classes share the same import; only the one referencing it should connect.
        let src = r#"
import { Logger } from "./logger";
import { Cache } from "./cache";

export class Alpha {
  use(): void {
    const log = new Logger();
    log.info("hi");
  }
}

export class Beta {
  add(value: number): number {
    return value + 1;
  }
}
"#;
        let mut graph = Graph::new();
        parse_typescript(&mut graph, "alpha.ts", src);
        let edges: Vec<&Edge> = graph
            .edges
            .iter()
            .filter(|e| matches!(e.kind, EdgeKind::Imports))
            .collect();
        let alpha_to_logger = edges
            .iter()
            .any(|e| e.from == "type:Alpha" && e.to.ends_with("Logger"));
        let beta_to_logger = edges
            .iter()
            .any(|e| e.from == "type:Beta" && e.to.ends_with("Logger"));
        let beta_to_cache = edges
            .iter()
            .any(|e| e.from == "type:Beta" && e.to.ends_with("Cache"));
        assert!(alpha_to_logger, "Alpha should import Logger");
        assert!(!beta_to_logger, "Beta must not import Logger");
        assert!(!beta_to_cache, "Beta must not import unused Cache");
    }

    #[test]
    fn test_compute_metrics_basic() {
        let mut graph = Graph::new();
        graph.upsert_node(Node {
            id: "type:A".into(),
            name: "A".into(),
            kind: NodeKind::Type,
            path: "a.ts".into(),
            layer: Some("service".into()),
        });
        graph.upsert_node(Node {
            id: "type:B".into(),
            name: "B".into(),
            kind: NodeKind::Type,
            path: "b.ts".into(),
            layer: Some("data".into()),
        });
        graph.add_edge("type:A", "type:B", EdgeKind::Imports);
        graph.add_edge("type:B", "type:A", EdgeKind::Imports);
        let m = compute_metrics(&graph);
        assert_eq!(m.node_count, 2);
        assert_eq!(m.cycles.len(), 1);
        assert!(m.layer_mix.get("service").copied().unwrap_or(0) >= 1);
    }
}
