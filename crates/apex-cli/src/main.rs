use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

fn main() {
    if let Err(error) = run() {
        eprintln!("apex: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };
    match command.as_str() {
        "init" => init_workspace(Path::new(".")).map_err(|error| error.to_string()),
        "scan" => {
            let root = args.next().unwrap_or_else(|| ".".to_string());
            let root_path = expand_path_shorthand(&root)?;
            let graph =
                apex_core::parse_repository(&root_path).map_err(|error| error.to_string())?;
            println!("{}", graph.to_json());
            Ok(())
        }
        "check" => {
            let options = CheckOptions::from_args(args.collect())?;
            let root_path = expand_path_shorthand(&options.root)?;
            let graph =
                apex_core::parse_repository(&root_path).map_err(|error| error.to_string())?;
            let rules = load_rules_for_cli(&root_path, options.rules.as_deref())?;
            let violations = apex_core::check_graph_with_rules(&graph, &rules);
            if violations.is_empty() {
                println!("Apex check passed: no violations");
                Ok(())
            } else {
                for violation in &violations {
                    println!(
                        "{}: {} ({})",
                        violation.rule_id, violation.message, violation.subject
                    );
                }
                Err(format!("{} violation(s) detected", violations.len()))
            }
        }
        "serve" => {
            let root = args.next().unwrap_or_else(|| ".".to_string());
            let root_path = expand_path_shorthand(&root)?;
            let status = apexd::serve_once(&root_path).map_err(|error| error.to_string())?;
            println!(
                "Apex daemon scan ready for {} with {} graph nodes; run `apex ui` for the local workbench",
                status.workspace, status.nodes
            );
            Ok(())
        }
        "export" => {
            let options = ExportOptions::from_args(args.collect())?;
            export_diagram(options)
        }
        "diagram" => {
            let options = ExportOptions::from_args(args.collect())?;
            export_diagram(options)
        }
        "ui" => {
            let options = UiOptions::from_args(args.collect())?;
            serve_ui(options)
        }
        "rules" => handle_rules(args.collect()),
        "metrics" => {
            let options = MetricsOptions::from_args(args.collect())?;
            let root_path = expand_path_shorthand(&options.root)?;
            let graph =
                apex_core::parse_repository(&root_path).map_err(|error| error.to_string())?;
            let metrics = apex_core::compute_metrics(&graph);
            if options.format == "json" {
                println!("{}", apex_core::metrics_to_json(&metrics));
            } else {
                print_metrics_text(&metrics);
            }
            Ok(())
        }
        "languages" => {
            print_languages();
            Ok(())
        }
        "capabilities" => {
            print_capabilities();
            Ok(())
        }
        "docs" => {
            print_docs_index();
            Ok(())
        }
        "--help" | "-h" | "help" => {
            print_help();
            Ok(())
        }
        other => Err(format!("unknown command '{other}'")),
    }
}

fn init_workspace(root: &Path) -> io::Result<()> {
    write_if_missing(
        root.join(".apex/config.yaml"),
        "version: 1\nstore: .apex/graph\n",
    )?;
    write_if_missing(root.join(".apex/overrides/.gitkeep"), "")?;
    write_if_missing(root.join(".apex/diagrams/.gitkeep"), "")?;
    write_if_missing(
        root.join(".apex/lenses/default.yaml"),
        "name: default\ninclude: ['*']\nhops: 2\n",
    )?;
    write_if_missing(
        root.join("apex.rules.yaml"),
        "version: 1\nrules:\n  - id: RULE-LAYER-001\n    type: forbidden_import\n    from: api\n    to: infrastructure\n",
    )?;
    write_if_missing(
        root.join("apex.workspace.yaml"),
        "version: 1\nroots:\n  - .\n",
    )?;
    println!("Initialized Apex workspace");
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct CheckOptions {
    root: String,
    rules: Option<String>,
}

impl CheckOptions {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        let mut root = ".".to_string();
        let mut rules = None;
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--rules" | "-r" => {
                    index += 1;
                    rules = Some(
                        args.get(index)
                            .ok_or_else(|| "--rules requires a file path".to_string())?
                            .clone(),
                    );
                }
                "--help" | "-h" => {
                    return Err("usage: apex check [path] [--rules apex.rules.yaml]".to_string());
                }
                value => root = value.to_string(),
            }
            index += 1;
        }
        Ok(Self { root, rules })
    }
}

fn load_rules_for_cli(
    root: &Path,
    rules_path: Option<&str>,
) -> Result<Vec<apex_core::RuleDefinition>, String> {
    let path = rules_path
        .map(expand_path_shorthand)
        .transpose()?
        .unwrap_or_else(|| root.join("apex.rules.yaml"));
    if path.exists() {
        apex_core::load_rules(&path)
            .map_err(|error| format!("failed to load '{}': {error}", path.display()))
    } else {
        Ok(apex_core::default_rules())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MetricsOptions {
    root: String,
    format: String,
}

impl MetricsOptions {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        let mut root = ".".to_string();
        let mut format = "text".to_string();
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--format" | "-f" => {
                    index += 1;
                    format = args
                        .get(index)
                        .ok_or_else(|| "--format requires a value".to_string())?
                        .clone();
                }
                "--help" | "-h" => {
                    return Err("usage: apex metrics [path] [--format text|json]".to_string());
                }
                value => root = value.to_string(),
            }
            index += 1;
        }
        if format != "text" && format != "json" {
            return Err(format!("unsupported metrics format '{format}'"));
        }
        Ok(Self { root, format })
    }
}

fn print_metrics_text(metrics: &apex_core::GraphMetrics) {
    println!("nodes:      {}", metrics.node_count);
    println!("edges:      {}", metrics.edge_count);
    println!("components: {}", metrics.component_count);
    println!("cycles:     {}", metrics.cycles.len());
    if !metrics.layer_mix.is_empty() {
        println!("\nlayers:");
        for (layer, count) in &metrics.layer_mix {
            println!("  {layer:<16} {count}");
        }
    }
    if !metrics.layer_edges.is_empty() {
        println!("\nlayer edges:");
        for ((from, to), count) in &metrics.layer_edges {
            println!("  {from} -> {to}  ({count})");
        }
    }
    if !metrics.hotspots.is_empty() {
        println!("\nhotspots (fan_in / fan_out):");
        for hot in &metrics.hotspots {
            println!("  {:<32} in={} out={}", hot.name, hot.fan_in, hot.fan_out);
        }
    }
    if !metrics.cycles.is_empty() {
        println!("\nimport cycles:");
        for (i, cycle) in metrics.cycles.iter().enumerate() {
            println!("  {}. {}", i + 1, cycle.join(" -> "));
        }
    }
    if !metrics.orphans.is_empty() {
        println!("\norphans (disconnected types): {}", metrics.orphans.len());
        for id in metrics.orphans.iter().take(20) {
            println!("  {id}");
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ExportOptions {
    format: String,
    root: String,
    output: Option<String>,
}

impl ExportOptions {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        let mut format = "svg".to_string();
        let mut root = ".".to_string();
        let mut output = None;
        let mut positional = Vec::new();
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--format" | "-f" => {
                    index += 1;
                    format = args
                        .get(index)
                        .ok_or_else(|| "--format requires a value".to_string())?
                        .clone();
                }
                "--out" | "-o" => {
                    index += 1;
                    output = Some(
                        args.get(index)
                            .ok_or_else(|| "--out requires a value".to_string())?
                            .to_string(),
                    );
                }
                "--help" | "-h" => {
                    return Err(
                        "usage: apex diagram [path] [--format svg|mermaid|html|json] [--out file]"
                            .to_string(),
                    );
                }
                value => positional.push(value.to_string()),
            }
            index += 1;
        }
        for value in &positional {
            if is_format(value) {
                format = value.clone();
            } else {
                root = value.clone();
            }
        }
        if !is_format(&format) {
            return Err(format!("unsupported export format '{format}'"));
        }
        Ok(Self {
            format,
            root,
            output,
        })
    }
}

fn export_diagram(options: ExportOptions) -> Result<(), String> {
    let root_path = expand_path_shorthand(&options.root)?;
    let graph = apex_core::parse_repository(&root_path)
        .map_err(|error| format!("failed to scan '{}': {error}", options.root))?;
    let content = match options.format.as_str() {
        "json" => graph.to_json(),
        "mermaid" => graph.to_mermaid(),
        "svg" => graph.to_svg(),
        "html" => graph_to_html(&graph),
        _ => unreachable!("format was validated"),
    };
    if let Some(output) = options.output {
        let path = expand_path_shorthand(&output)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create '{}': {error}", parent.display()))?;
        }
        fs::write(&path, content)
            .map_err(|error| format!("failed to write '{}': {error}", path.display()))?;
        println!("Wrote {}", path.display());
    } else {
        println!("{content}");
    }
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct UiOptions {
    host: String,
    port: u16,
    ui_dist: PathBuf,
}

impl UiOptions {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        let mut host = "127.0.0.1".to_string();
        let mut port = 4317u16;
        let mut ui_dist = "ui/dist".to_string();
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--host" => {
                    index += 1;
                    host = args
                        .get(index)
                        .ok_or_else(|| "--host requires a value".to_string())?
                        .clone();
                }
                "--port" | "-p" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "--port requires a value".to_string())?;
                    port = value
                        .parse()
                        .map_err(|_| format!("invalid port '{value}'"))?;
                }
                "--ui-dist" => {
                    index += 1;
                    ui_dist = args
                        .get(index)
                        .ok_or_else(|| "--ui-dist requires a value".to_string())?
                        .clone();
                }
                "--help" | "-h" => {
                    return Err(
                        "usage: apex ui [--host 127.0.0.1] [--port 4317] [--ui-dist ui/dist]"
                            .to_string(),
                    );
                }
                value => return Err(format!("unknown ui option '{value}'")),
            }
            index += 1;
        }
        Ok(Self {
            host,
            port,
            ui_dist: expand_path_shorthand(&ui_dist)?,
        })
    }
}

fn expand_path_shorthand(input: &str) -> Result<PathBuf, String> {
    let value = input.trim();
    if value.is_empty() {
        return Ok(PathBuf::from("."));
    }
    if value == "~" {
        return home_dir().ok_or_else(|| "cannot expand '~' because HOME is not set".to_string());
    }
    if let Some(rest) = value.strip_prefix("~/") {
        return home_dir()
            .map(|home| home.join(rest))
            .ok_or_else(|| "cannot expand '~/' because HOME is not set".to_string());
    }
    if value == "$HOME" || value == "${HOME}" {
        return home_dir()
            .ok_or_else(|| "cannot expand HOME shorthand because HOME is not set".to_string());
    }
    if let Some(rest) = value.strip_prefix("$HOME/") {
        return home_dir()
            .map(|home| home.join(rest))
            .ok_or_else(|| "cannot expand HOME shorthand because HOME is not set".to_string());
    }
    if let Some(rest) = value.strip_prefix("${HOME}/") {
        return home_dir()
            .map(|home| home.join(rest))
            .ok_or_else(|| "cannot expand HOME shorthand because HOME is not set".to_string());
    }
    Ok(PathBuf::from(value))
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

fn serve_ui(options: UiOptions) -> Result<(), String> {
    let index_path = options.ui_dist.join("index.html");
    if !index_path.exists() {
        return Err(format!(
            "UI build not found at '{}'; run `npm run build:ui` first",
            options.ui_dist.display()
        ));
    }
    let address = format!("{}:{}", options.host, options.port);
    let listener = TcpListener::bind(&address)
        .map_err(|error| format!("failed to bind UI server at {address}: {error}"))?;
    println!("Apex UI running at http://{address}");
    println!("Serving UI build from {}", options.ui_dist.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_ui_connection(stream, &options.ui_dist) {
                    eprintln!("apex ui request failed: {error}");
                }
            }
            Err(error) => eprintln!("apex ui connection failed: {error}"),
        }
    }
    Ok(())
}

fn handle_ui_connection(mut stream: TcpStream, ui_dist: &Path) -> Result<(), String> {
    let mut buffer = [0_u8; 8192];
    let bytes = stream
        .read(&mut buffer)
        .map_err(|error| format!("failed to read request: {error}"))?;
    if bytes == 0 {
        return Ok(());
    }
    let request = String::from_utf8_lossy(&buffer[..bytes]);
    let Some(request_line) = request.lines().next() else {
        write_response(
            &mut stream,
            400,
            "text/plain; charset=utf-8",
            "missing request line",
        )?;
        return Ok(());
    };
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        write_response(
            &mut stream,
            405,
            "application/json; charset=utf-8",
            "{\"error\":\"method not allowed\"}",
        )?;
        return Ok(());
    }
    if target.starts_with("/api/") {
        let response = handle_api_route(target);
        write_response(
            &mut stream,
            response.status,
            response.content_type,
            &response.body,
        )?;
        return Ok(());
    }
    serve_static_route(&mut stream, ui_dist, target)
}

struct HttpResponse {
    status: u16,
    content_type: &'static str,
    body: String,
}

fn handle_api_route(target: &str) -> HttpResponse {
    let (path, query) = split_target(target);
    match path {
        "/api/health" => HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body: "{\"status\":\"ok\"}".to_string(),
        },
        "/api/scan" => graph_for_query(query).map_or_else(error_response, |graph| HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body: graph.to_json(),
        }),
        "/api/check" => graph_for_query(query).map_or_else(error_response, |graph| HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body: violations_to_json(&apex_core::check_graph(&graph)),
        }),
        "/api/metrics" => {
            graph_for_query(query).map_or_else(error_response, |graph| HttpResponse {
                status: 200,
                content_type: "application/json; charset=utf-8",
                body: apex_core::metrics_to_json(&apex_core::compute_metrics(&graph)),
            })
        }
        "/api/rules" => HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body: rules_to_json(&apex_core::default_rules()),
        },
        "/api/languages" => HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body: languages_to_json(),
        },
        "/api/diagram" => {
            let params = parse_query(query);
            let format = params
                .iter()
                .find(|(key, _)| key == "format")
                .map(|(_, value)| value.as_str())
                .unwrap_or("svg");
            if !is_format(format) {
                return error_response(format!("unsupported diagram format '{format}'"));
            }
            graph_for_params(&params).map_or_else(error_response, |graph| {
                let (content_type, body) = match format {
                    "json" => ("application/json; charset=utf-8", graph.to_json()),
                    "mermaid" => ("text/plain; charset=utf-8", graph.to_mermaid()),
                    "html" => ("text/html; charset=utf-8", graph_to_html(&graph)),
                    "svg" => ("image/svg+xml; charset=utf-8", graph.to_svg()),
                    _ => unreachable!("format was validated"),
                };
                HttpResponse {
                    status: 200,
                    content_type,
                    body,
                }
            })
        }
        _ => error_response("unknown API route".to_string()),
    }
}

fn serve_static_route(stream: &mut TcpStream, ui_dist: &Path, target: &str) -> Result<(), String> {
    let path_only = split_target(target).0;
    let relative = if path_only == "/" {
        PathBuf::from("index.html")
    } else {
        let decoded = percent_decode(path_only.trim_start_matches('/'))?;
        if decoded.split('/').any(|segment| segment == "..") {
            write_response(
                stream,
                403,
                "text/plain; charset=utf-8",
                "path traversal is not allowed",
            )?;
            return Ok(());
        }
        PathBuf::from(decoded)
    };
    let path = ui_dist.join(relative);
    let final_path = if path.is_file() {
        path
    } else {
        ui_dist.join("index.html")
    };
    let body = fs::read(&final_path)
        .map_err(|error| format!("failed to read '{}': {error}", final_path.display()))?;
    write_binary_response(stream, 200, content_type_for(&final_path), &body)
}

fn graph_for_query(query: &str) -> Result<apex_core::Graph, String> {
    let params = parse_query(query);
    graph_for_params(&params)
}

fn graph_for_params(params: &[(String, String)]) -> Result<apex_core::Graph, String> {
    let root = params
        .iter()
        .find(|(key, _)| key == "path")
        .map(|(_, value)| value.as_str())
        .unwrap_or(".");
    let root_path = expand_path_shorthand(root)?;
    apex_core::parse_repository(&root_path)
        .map_err(|error| format!("failed to scan '{root}': {error}"))
}

fn split_target(target: &str) -> (&str, &str) {
    target
        .split_once('?')
        .map_or((target, ""), |(path, query)| (path, query))
}

fn parse_query(query: &str) -> Vec<(String, String)> {
    query
        .split('&')
        .filter(|part| !part.is_empty())
        .filter_map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            Some((percent_decode(key).ok()?, percent_decode(value).ok()?))
        })
        .collect()
}

fn percent_decode(value: &str) -> Result<String, String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' => {
                if index + 2 >= bytes.len() {
                    return Err("incomplete percent escape".to_string());
                }
                let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                    .map_err(|_| "invalid percent escape".to_string())?;
                let byte = u8::from_str_radix(hex, 16)
                    .map_err(|_| format!("invalid percent escape '%{hex}'"))?;
                decoded.push(byte);
                index += 3;
            }
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(decoded).map_err(|_| "decoded query is not valid UTF-8".to_string())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> Result<(), String> {
    write_binary_response(stream, status, content_type, body.as_bytes())
}

fn write_binary_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<(), String> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(header.as_bytes())
        .and_then(|()| stream.write_all(body))
        .map_err(|error| format!("failed to write response: {error}"))
}

fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("svg") => "image/svg+xml; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    }
}

fn error_response(message: String) -> HttpResponse {
    HttpResponse {
        status: 400,
        content_type: "application/json; charset=utf-8",
        body: format!("{{\"error\":\"{}\"}}", escape_json(&message)),
    }
}

fn violations_to_json(violations: &[apex_core::Violation]) -> String {
    let mut body = String::from("[");
    for (index, violation) in violations.iter().enumerate() {
        if index > 0 {
            body.push(',');
        }
        body.push_str(&format!(
            "{{\"rule_id\":\"{}\",\"message\":\"{}\",\"subject\":\"{}\"}}",
            escape_json(&violation.rule_id),
            escape_json(&violation.message),
            escape_json(&violation.subject)
        ));
    }
    body.push(']');
    body
}

fn rules_to_json(rules: &[apex_core::RuleDefinition]) -> String {
    let mut body = String::from("[");
    for (index, rule) in rules.iter().enumerate() {
        if index > 0 {
            body.push(',');
        }
        body.push_str(&format!(
            "{{\"id\":\"{}\",\"type\":\"{}\",\"from\":{},\"to\":{},\"enabled\":{}}}",
            escape_json(&rule.id),
            escape_json(&rule.rule_type),
            optional_json(rule.from.as_deref()),
            optional_json(rule.to.as_deref()),
            rule.enabled
        ));
    }
    body.push(']');
    body
}

fn languages_to_json() -> String {
    let languages = apex_core::supported_languages();
    let mut body = String::from("[");
    for (index, language) in languages.iter().enumerate() {
        if index > 0 {
            body.push(',');
        }
        let extensions = language
            .extensions
            .iter()
            .map(|extension| format!("\"{}\"", escape_json(extension)))
            .collect::<Vec<_>>()
            .join(",");
        body.push_str(&format!(
            "{{\"name\":\"{}\",\"extensions\":[{}],\"extracts\":\"{}\"}}",
            escape_json(language.name),
            extensions,
            escape_json(language.extracts)
        ));
    }
    body.push(']');
    body
}

fn optional_json(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", escape_json(value)))
        .unwrap_or_else(|| "null".to_string())
}

fn graph_to_html(graph: &apex_core::Graph) -> String {
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>Apex Diagram</title><style>{}</style></head><body><main><header><h1>Apex Diagram</h1><p>{} nodes · {} edges</p></header><section class=\"diagram\">{}</section><details><summary>Graph JSON</summary><pre>{}</pre></details></main></body></html>",
        "body{margin:0;font-family:Inter,system-ui,sans-serif;background:#0d1117;color:#e6edf3}main{padding:24px}header{margin-bottom:16px}.diagram{overflow:auto;background:#fff;border-radius:12px;padding:24px}svg{max-width:none}details{margin-top:16px}pre{overflow:auto;background:#161b22;padding:16px;border-radius:8px}",
        graph.nodes.len(),
        graph.edges.len(),
        graph.to_svg(),
        escape_html(&graph.to_json())
    )
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

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn is_format(value: &str) -> bool {
    matches!(value, "svg" | "mermaid" | "html" | "json")
}

fn handle_rules(args: Vec<String>) -> Result<(), String> {
    match args.first().map(String::as_str).unwrap_or("list") {
        "list" => {
            for rule in apex_core::default_rules() {
                println!(
                    "{} ({}) enabled={} from={} to={}",
                    rule.id,
                    rule.rule_type,
                    rule.enabled,
                    rule.from.as_deref().unwrap_or("-"),
                    rule.to.as_deref().unwrap_or("-")
                );
            }
            Ok(())
        }
        "explain" => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            print_rule_explanation(id);
            Ok(())
        }
        "template" => {
            if let Some(out_index) = args.iter().position(|arg| arg == "--out" || arg == "-o") {
                let path = args
                    .get(out_index + 1)
                    .ok_or_else(|| "--out requires a file path".to_string())?;
                let output_path = expand_path_shorthand(path)?;
                fs::write(&output_path, rules_template()).map_err(|error| {
                    format!("failed to write '{}': {error}", output_path.display())
                })?;
                println!("Wrote {}", output_path.display());
            } else {
                println!("{}", rules_template());
            }
            Ok(())
        }
        "--help" | "-h" | "help" => {
            println!("apex rules <list|explain|template> [--out apex.rules.yaml]");
            Ok(())
        }
        other => Err(format!("unknown rules command '{other}'")),
    }
}

fn print_rule_explanation(id: &str) {
    match id {
        "RULE-LAYER-001" | "forbidden_import" => println!(
            "RULE-LAYER-001 prevents one architectural layer from importing another. Configure it with type: forbidden_import, from: <layer>, to: <layer>."
        ),
        "RULE-CYCLE-001" | "import_cycle" => println!(
            "RULE-CYCLE-001 detects import cycles in the parsed dependency graph. Configure it with type: import_cycle."
        ),
        _ => println!("Known rules: RULE-LAYER-001, RULE-CYCLE-001"),
    }
}

fn rules_template() -> &'static str {
    "version: 1\nrules:\n  - id: RULE-LAYER-001\n    type: forbidden_import\n    from: api\n    to: infrastructure\n    enabled: true\n  - id: RULE-CYCLE-001\n    type: import_cycle\n    enabled: true\n"
}

fn print_languages() {
    println!("Supported languages and file types:");
    for language in apex_core::supported_languages() {
        println!(
            "- {} ({}) — {}",
            language.name,
            language.extensions.join(", "),
            language.extracts
        );
    }
}

fn print_capabilities() {
    println!("Apex can scan repositories, build architecture graphs, check rules, generate SVG/Mermaid/HTML/JSON diagrams, run a local UI, and expose VS Code rendering integration.");
}

fn print_docs_index() {
    println!("User docs live in ./docs:\n- docs/getting-started.md\n- docs/cli.md\n- docs/ui.md\n- docs/rules.md\n- docs/languages.md\n- docs/diagrams.md\n- docs/configuration.md\n- docs/vscode.md\n- docs/troubleshooting.md");
}

fn write_if_missing(path: PathBuf, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

fn print_help() {
    println!(
        "apex <command>\n\nCommands:\n  init\n  scan [path]\n  check [path] [--rules apex.rules.yaml]\n  metrics [path] [--format text|json]\n  serve [path]\n  export [format] [path] [--out file]\n  diagram [path] [--format svg|mermaid|html|json] [--out file]\n  ui [--host 127.0.0.1] [--port 4317] [--ui-dist ui/dist]\n  rules <list|explain|template>\n  languages\n  capabilities\n  docs\n  help"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_path_shorthand_supports_tilde() {
        let expanded = expand_path_shorthand("~/").expect("expand ~/");
        assert!(expanded.is_absolute());
    }

    #[test]
    fn test_expand_path_shorthand_supports_home_env_forms() {
        let expanded = expand_path_shorthand("$HOME").expect("expand $HOME");
        assert!(expanded.is_absolute());
        let expanded_braced = expand_path_shorthand("${HOME}/tmp").expect("expand ${HOME}/tmp");
        assert!(expanded_braced.ends_with("tmp"));
    }
}
