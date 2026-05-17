# Rules guide

Apex rules live in `apex.rules.yaml`.

Create a starter file:

```bash
cargo run -p apex-cli -- rules template --out apex.rules.yaml
```

## Supported rule types

### `forbidden_import`

Prevents one layer from importing another.

```yaml
version: 1
rules:
  - id: RULE-LAYER-001
    type: forbidden_import
    from: api
    to: infrastructure
    enabled: true
```

This catches direct import edges where the source node is in the `api` layer and the target node is in the `infrastructure` layer.

### `import_cycle`

Detects import cycles in the parsed graph.

```yaml
version: 1
rules:
  - id: RULE-CYCLE-001
    type: import_cycle
    enabled: true
```

## Layer detection

Apex infers layers from file paths and common names:

| Layer | Detected from |
|---|---|
| `api` | `api/`, `controller` |
| `service` | `service`, `domain/` |
| `data` | `repository`, `data/`, `db/` |
| `infrastructure` | `infrastructure/`, `infra/` |
| `config` | manifest/config files |

## Run rules

```bash
cargo run -p apex-cli -- check .
cargo run -p apex-cli -- check . --rules apex.rules.yaml
```

