# Configuration

## `apex.workspace.yaml`

Defines workspace roots.

```yaml
version: 1
roots:
  - .
```

## `apex.rules.yaml`

Defines architecture rules. See [rules.md](rules.md).

## `.apex/lenses/default.yaml`

Stores local view preferences.

```yaml
name: default
include: ["*"]
hops: 2
```

## `.apex/overrides/`

Reserved for local graph override files. Overrides are intended for manually annotating graph data without changing source files.

## `.state/`

Internal generated state and old build notes live here. It is ignored by Git and should not be used for user-facing documentation.

