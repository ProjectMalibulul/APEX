# CLI reference

## `apex init`

Creates local Apex config files:

- `.apex/config.yaml`
- `.apex/lenses/default.yaml`
- `.apex/overrides/.gitkeep`
- `.apex/diagrams/.gitkeep`
- `apex.rules.yaml`
- `apex.workspace.yaml`

## `apex languages`

Prints supported languages and what Apex extracts.

## `apex capabilities`

Prints a compact summary of what the current build can do.

## `apex scan [path]`

Scans a repository and prints graph JSON.

## `apex check [path] [--rules apex.rules.yaml]`

Scans a repository and runs architecture rules. Exits non-zero when violations are found.

## `apex rules list`

Lists built-in rules.

## `apex rules explain RULE-LAYER-001`

Explains a rule and how to configure it.

## `apex rules template --out apex.rules.yaml`

Writes a starter rules file.

## `apex diagram [path] --format svg|mermaid|html|json --out file`

Generates a diagram or graph file locally.

## `apex ui [--host 127.0.0.1] [--port 4317]`

Serves the Vite-built Apex workbench and local API routes.

