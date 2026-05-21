# Metrics

`apex metrics` computes architectural signals from the parsed graph without leaving your machine.

## Usage

```bash
apex metrics [path] [--format text|json]
```

- Default `path` is the current directory.
- Default `--format` is `text` for humans; `json` is suited for tooling (and is what the UI fetches from `/api/metrics`).

## What you get

| Field             | Meaning                                                                 |
| ----------------- | ----------------------------------------------------------------------- |
| `node_count`      | Total nodes in the graph (files, types, entities, manifests).           |
| `edge_count`      | Total edges (Imports, Extends, Implements, RelatesTo, Contains).        |
| `component_count` | Connected components over the diagram subgraph (types/entities only).   |
| `hotspots`        | Top 10 nodes ranked by `fan_in + fan_out`, with a name and counts.      |
| `cycles`          | Import cycles between types, deduplicated by rotation.                  |
| `orphans`         | Type/entity nodes with no edges in or out.                              |
| `layer_mix`       | Count of nodes per detected layer (api, service, data, …).              |
| `layer_edges`     | Count of edges between layers, useful for spotting layering breakages.  |

## In the UI

Click the **Metrics** button in the canvas toolbar after running **Scan repository**. The panel shows the
counts, hotspots, cycles, and layer mix for the currently selected repository path.

## In CI

The JSON output is stable for tooling. A simple cycle-budget check looks like:

```bash
apex metrics . --format json | jq '.cycles | length' \
  | xargs -I{} test {} -le 0
```
