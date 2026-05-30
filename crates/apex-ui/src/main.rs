use rust_embed::Embed;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

/// All files from `ui/dist/` are baked into the binary at compile time.
/// Run `npm run build:ui` from the workspace root before building this crate.
#[derive(Embed)]
#[folder = "../../ui/dist"]
struct UiAssets;

fn main() {
    let listener = bind_free_listener(4322);
    let port = listener
        .local_addr()
        .map(|addr| addr.port())
        .unwrap_or(4322);
    let url = format!("http://127.0.0.1:{port}");
    println!("Apex  →  {url}");
    println!("Press Ctrl+C to stop.");

    let url_open = url.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(400));
        if let Err(e) = open::that(&url_open) {
            eprintln!("apex-ui: could not open browser: {e}");
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(|| handle_connection(s));
            }
            Err(e) => eprintln!("apex-ui: connection error: {e}"),
        }
    }
}

fn bind_free_listener(start: u16) -> TcpListener {
    for port in start.. {
        if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{port}")) {
            return listener;
        }
    }
    panic!("no free port found starting from {start}");
}

// ── HTTP handling ────────────────────────────────────────────────────────────

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 16384];
    let Ok(n) = stream.read(&mut buf) else { return };
    if n == 0 {
        return;
    }
    let req = String::from_utf8_lossy(&buf[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let _method = parts.next().unwrap_or("GET");
    let target = parts.next().unwrap_or("/");
    let (path, query) = split_target(target);

    if path.starts_with("/api/") {
        let resp = handle_api_route(path, query);
        let _ = write_response(&mut stream, resp.0, resp.1, &resp.2);
    } else {
        let _ = serve_embedded(&mut stream, path);
    }
}

fn serve_embedded(stream: &mut TcpStream, path: &str) -> std::io::Result<()> {
    let asset_path = if path == "/" {
        "index.html".to_string()
    } else {
        path.trim_start_matches('/').to_string()
    };
    let (data, mime): (Vec<u8>, &str) = match UiAssets::get(&asset_path) {
        Some(f) => (f.data.into_owned(), mime_for(&asset_path)),
        None => {
            // SPA fallback: return index.html for unknown routes
            match UiAssets::get("index.html") {
                Some(f) => (f.data.into_owned(), "text/html; charset=utf-8"),
                None => {
                    let _ = write_response(stream, 404, "text/plain", "not found");
                    return Ok(());
                }
            }
        }
    };
    write_binary_response(stream, 200, mime, &data)
}

fn mime_for(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "text/javascript; charset=utf-8"
    } else if path.ends_with(".svg") {
        "image/svg+xml; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else {
        "application/octet-stream"
    }
}

// ── API routes ───────────────────────────────────────────────────────────────

fn handle_api_route(path: &str, query: &str) -> (u16, &'static str, String) {
    match path {
        "/api/health" => (
            200,
            "application/json; charset=utf-8",
            "{\"status\":\"ok\"}".to_string(),
        ),
        "/api/scan" => match graph_for_query(query) {
            Ok(g) => (200, "application/json; charset=utf-8", g.to_json()),
            Err(e) => error_response(e),
        },
        "/api/check" => match graph_for_query(query) {
            Ok(g) => (
                200,
                "application/json; charset=utf-8",
                violations_to_json(&apex_core::check_graph(&g)),
            ),
            Err(e) => error_response(e),
        },
        "/api/metrics" => match graph_for_query(query) {
            Ok(g) => (
                200,
                "application/json; charset=utf-8",
                apex_core::metrics_to_json(&apex_core::compute_metrics(&g)),
            ),
            Err(e) => error_response(e),
        },
        "/api/rules" => (
            200,
            "application/json; charset=utf-8",
            rules_to_json(&apex_core::default_rules()),
        ),
        "/api/languages" => (200, "application/json; charset=utf-8", languages_to_json()),
        "/api/diagram" => {
            let params = parse_query(query);
            let format = params
                .iter()
                .find(|(k, _)| k == "format")
                .map(|(_, v)| v.as_str())
                .unwrap_or("svg");
            if !is_format(format) {
                return error_response(format!("unsupported diagram format '{format}'"));
            }
            match graph_for_params(&params) {
                Ok(g) => {
                    let (ct, body) = match format {
                        "json" => ("application/json; charset=utf-8", g.to_json()),
                        "mermaid" => ("text/plain; charset=utf-8", g.to_mermaid()),
                        "svg" => ("image/svg+xml; charset=utf-8", g.to_svg()),
                        "html" => ("text/html; charset=utf-8", graph_to_html(&g)),
                        _ => unreachable!(),
                    };
                    (200, ct, body)
                }
                Err(e) => error_response(e),
            }
        }
        _ => error_response("unknown API route".to_string()),
    }
}

fn error_response(msg: String) -> (u16, &'static str, String) {
    (
        400,
        "application/json; charset=utf-8",
        format!("{{\"error\":\"{}\"}}", escape_json(&msg)),
    )
}

fn graph_for_query(query: &str) -> Result<apex_core::Graph, String> {
    graph_for_params(&parse_query(query))
}

fn graph_for_params(params: &[(String, String)]) -> Result<apex_core::Graph, String> {
    let root = params
        .iter()
        .find(|(k, _)| k == "path")
        .map(|(_, v)| v.as_str())
        .unwrap_or(".");
    let root_path = expand_path(root)?;
    apex_core::parse_repository(&root_path)
        .map_err(|e| format!("failed to scan '{root}': {e}"))
}

// ── Path helpers ─────────────────────────────────────────────────────────────

fn expand_path(input: &str) -> Result<PathBuf, String> {
    let s = input.trim();
    if s.is_empty() {
        return Ok(PathBuf::from("."));
    }
    if s == "~" {
        return home_dir().ok_or_else(|| "HOME is not set".to_string());
    }
    if let Some(rest) = s.strip_prefix("~/") {
        return home_dir()
            .map(|h| h.join(rest))
            .ok_or_else(|| "HOME is not set".to_string());
    }
    if s == "$HOME" || s == "${HOME}" {
        return home_dir().ok_or_else(|| "HOME is not set".to_string());
    }
    if let Some(rest) = s.strip_prefix("$HOME/").or_else(|| s.strip_prefix("${HOME}/")) {
        return home_dir()
            .map(|h| h.join(rest))
            .ok_or_else(|| "HOME is not set".to_string());
    }
    Ok(PathBuf::from(s))
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

// ── Serialisation helpers ────────────────────────────────────────────────────

fn violations_to_json(violations: &[apex_core::Violation]) -> String {
    let items: Vec<String> = violations
        .iter()
        .map(|v| {
            format!(
                "{{\"rule_id\":\"{}\",\"message\":\"{}\",\"subject\":\"{}\"}}",
                escape_json(&v.rule_id),
                escape_json(&v.message),
                escape_json(&v.subject)
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn rules_to_json(rules: &[apex_core::RuleDefinition]) -> String {
    let items: Vec<String> = rules
        .iter()
        .map(|r| {
            format!(
                "{{\"id\":\"{}\",\"type\":\"{}\",\"from\":{},\"to\":{},\"enabled\":{}}}",
                escape_json(&r.id),
                escape_json(&r.rule_type),
                r.from.as_deref().map_or("null".to_string(), |v| format!("\"{}\"", escape_json(v))),
                r.to.as_deref().map_or("null".to_string(), |v| format!("\"{}\"", escape_json(v))),
                r.enabled
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn languages_to_json() -> String {
    let langs = apex_core::supported_languages();
    let items: Vec<String> = langs
        .iter()
        .map(|l| {
            let exts = l
                .extensions
                .iter()
                .map(|e| format!("\"{}\"", escape_json(e)))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"name\":\"{}\",\"extensions\":[{}],\"extracts\":\"{}\"}}",
                escape_json(l.name),
                exts,
                escape_json(l.extracts)
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
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

fn is_format(s: &str) -> bool {
    matches!(s, "svg" | "mermaid" | "json" | "html")
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

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ── Query parsing ────────────────────────────────────────────────────────────

fn split_target(target: &str) -> (&str, &str) {
    target
        .split_once('?')
        .map_or((target, ""), |(p, q)| (p, q))
}

fn parse_query(query: &str) -> Vec<(String, String)> {
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .filter_map(|p| {
            let (k, v) = p.split_once('=').unwrap_or((p, ""));
            Some((percent_decode(k).ok()?, percent_decode(v).ok()?))
        })
        .collect()
}

fn percent_decode(s: &str) -> Result<String, String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3])
                    .map_err(|_| "invalid escape".to_string())?;
                let b = u8::from_str_radix(hex, 16)
                    .map_err(|_| format!("bad hex '%{hex}'"))?;
                out.push(b);
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|_| "not valid UTF-8".to_string())
}

// ── HTTP response writers ────────────────────────────────────────────────────

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    write_binary_response(stream, status, content_type, body.as_bytes())
}

fn write_binary_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)
}
