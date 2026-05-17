use std::env;
use std::fs;
use std::io;
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
                "Apex daemon ready for {} with {} graph nodes; web UI available at http://127.0.0.1:4317",
                status.workspace, status.nodes
            );
            Ok(())
        }
        "export" => {
            let format = args.next().unwrap_or_else(|| "mermaid".to_string());
            let root = args.next().unwrap_or_else(|| ".".to_string());
            let graph =
                apex_core::parse_repository(Path::new(&root)).map_err(|error| error.to_string())?;
            match format.as_str() {
                "mermaid" => println!("{}", graph.to_mermaid()),
                "svg" => println!("{}", graph.to_svg()),
                other => return Err(format!("unsupported export format '{other}'")),
            }
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
    println!("apex <command>\n\nCommands:\n  init\n  scan [path]\n  check [path]\n  serve [path]\n  export <mermaid|svg> [path]");
}
