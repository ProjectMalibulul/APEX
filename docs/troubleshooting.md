# Troubleshooting

## `apex ui` says the UI build is missing

Run:

```bash
npm run build:ui
```

or:

```bash
just ui
```

## `apex check` exits non-zero

That means rule violations were found. Read each printed `RULE-*` line. For the sample repository, violations are intentional.

## The UI cannot connect to the API

Start the dedicated backend:

```bash
cargo run -p apex-cli -- ui
```

For Vite development, keep that command running and use `npm run dev:ui`.

## Diagrams are empty

Run `apex languages` and confirm your files use supported extensions. Also check that generated directories such as `target/`, `node_modules/`, and `.git/` are intentionally ignored.

