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
            let graph =
                apex_core::parse_repository(Path::new(&root)).map_err(|error| error.to_string())?;
            println!("{}", graph.to_json());
            Ok(())
        }
        "check" => {
            let root = args.next().unwrap_or_else(|| ".".to_string());
            let graph =
                apex_core::parse_repository(Path::new(&root)).map_err(|error| error.to_string())?;
            let violations = apex_core::check_graph(&graph);
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
            let status = apexd::serve_once(Path::new(&root)).map_err(|error| error.to_string())?;
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
struct ExportOptions {
    format: String,
    root: String,
    output: Option<PathBuf>,
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
                    output = Some(PathBuf::from(
                        args.get(index)
                            .ok_or_else(|| "--out requires a value".to_string())?,
                    ));
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
        if positional.len() == 1 {
            if is_format(&positional[0]) {
                format = positional[0].clone();
            } else {
                root = positional[0].clone();
            }
        } else if positional.len() >= 2 {
            format = positional[0].clone();
            root = positional[1].clone();
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
    let graph = apex_core::parse_repository(Path::new(&options.root))
        .map_err(|error| format!("failed to scan '{}': {error}", options.root))?;
    let content = match options.format.as_str() {
        "json" => graph.to_json(),
        "mermaid" => graph.to_mermaid(),
        "svg" => graph.to_svg(),
        "html" => graph_to_html(&graph),
        _ => unreachable!("format was validated"),
    };
    if let Some(path) = options.output {
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
        let mut ui_dist = PathBuf::from("ui/dist");
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
                    ui_dist = PathBuf::from(
                        args.get(index)
                            .ok_or_else(|| "--ui-dist requires a value".to_string())?,
                    );
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
            ui_dist,
        })
    }
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
    apex_core::parse_repository(Path::new(root))
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
    value.replace('\\', "\\\\").replace('"', "\\\"")
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
        "apex <command>\n\nCommands:\n  init\n  scan [path]\n  check [path]\n  serve [path]\n  export [format] [path] [--out file]\n  diagram [path] [--format svg|mermaid|html|json] [--out file]\n  ui [--host 127.0.0.1] [--port 4317] [--ui-dist ui/dist]"
    );
}
