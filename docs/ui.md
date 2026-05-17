# Local UI guide

Start the dedicated UI server:

```bash
just ui
```

Open `http://127.0.0.1:4317/`.

## Main actions

- **Scan repository**: calls `/api/scan` and loads graph JSON.
- **Run rules**: calls `/api/check` and shows violations.
- **Show rules**: calls `/api/rules` and lists built-in rules.
- **Show languages**: calls `/api/languages` and lists recognizers.
- **Render via API**: calls `/api/diagram` for SVG, Mermaid, HTML, or JSON.
- **Render diagram**: renders the JSON currently in the text area in the browser.
- **Import JSON**: loads graph JSON from a local file.
- **Download current output**: downloads the current SVG, Mermaid, HTML, or JSON output.

## Development mode

Run the backend and frontend separately:

```bash
cargo run -p apex-cli -- ui
npm run dev:ui
```

Vite serves the frontend at `http://127.0.0.1:5173/` and proxies `/api` to the Apex UI server.

