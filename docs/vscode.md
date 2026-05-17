# VS Code integration

The current extension module exposes a tested integration surface:

- activation
- CLI graph JSON parsing
- SVG class diagram rendering
- webview HTML rendering

Run the smoke test:

```bash
just vscode-smoke
```

The smoke test builds the CLI, scans `test-fixtures/sample-repo`, feeds graph JSON through the extension module, and verifies rendered webview HTML.

