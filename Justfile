set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    just --list

setup:
    npm install --no-audit --no-fund

fmt:
    cargo fmt --all
    gofmt -w collab/*.go

build:
    npm run build:ui
    cargo build --workspace
    npm run build:types

# Build both release binaries with embedded UI assets
build-release:
    npm run build:ui
    cargo build --release -p apex-cli
    cargo build --release -p apex-ui
    @echo "apex  → target/release/apex"
    @echo "apex-ui → target/release/apex-ui  (self-contained, opens browser)"

test:
    cargo test --workspace
    npm test
    cd collab && go test ./...

lint:
    cargo clippy --workspace -- -D warnings
    ./node_modules/.bin/tsc --strict
    cd collab && go vet ./...

diagram target="test-fixtures/sample-repo":
    mkdir -p artifacts
    cargo run -q -p apex-cli -- diagram {{target}} --format svg --out artifacts/apex.svg
    cargo run -q -p apex-cli -- diagram {{target}} --format mermaid --out artifacts/apex.mmd
    cargo run -q -p apex-cli -- diagram {{target}} --format html --out artifacts/apex.html
    cargo run -q -p apex-cli -- diagram {{target}} --format json --out artifacts/apex.json

ui:
    npm run build
    cargo run -q -p apex-cli -- ui

ui-dev:
    npm run dev:ui

ui-smoke:
    npm run build
    cargo build -p apex-cli
    timeout 10s {{justfile_directory()}}/target/debug/apex ui --port 4322 > /tmp/apex-ui-server.log 2>&1 &
    for attempt in {1..30}; do curl -fsSL http://127.0.0.1:4322/api/health >/tmp/apex-ui-health.json && break || sleep 0.2; done
    curl -fsSL "http://127.0.0.1:4322/api/scan?path={{justfile_directory()}}/test-fixtures/sample-repo" | grep -q "UserService"
    curl -fsSL "http://127.0.0.1:4322/api/check?path={{justfile_directory()}}/test-fixtures/sample-repo" | grep -q "RULE-LAYER-001"
    curl -fsSL "http://127.0.0.1:4322/api/metrics?path={{justfile_directory()}}/test-fixtures/sample-repo" | grep -q "hotspots"
    curl -fsSL "http://127.0.0.1:4322/api/rules" | grep -q "RULE-LAYER-001"
    curl -fsSL "http://127.0.0.1:4322/api/languages" | grep -q "Kotlin"
    curl -fsSL "http://127.0.0.1:4322/api/diagram?path={{justfile_directory()}}/test-fixtures/sample-repo&format=svg" | grep -q "<svg"
    curl -fsSL http://127.0.0.1:4322/ | grep -q "Apex Workbench"

metrics target="test-fixtures/sample-repo":
    cargo run -q -p apex-cli -- metrics {{target}}

release-local target="test-fixtures/sample-repo":
    npm run build:ui
    cargo build --release -p apex-cli
    cargo build --release -p apex-ui
    mkdir -p artifacts/local-release
    cp target/release/apex artifacts/local-release/ 2>/dev/null || cp target/release/apex.exe artifacts/local-release/
    cp target/release/apex-ui artifacts/local-release/ 2>/dev/null || cp target/release/apex-ui.exe artifacts/local-release/
    cp README.md LICENSE artifacts/local-release/ 2>/dev/null || true
    cp -R docs artifacts/local-release/ 2>/dev/null || true
    @echo "Local release ready in artifacts/local-release/"

vscode-smoke:
    cargo build -p apex-cli
    npm run test:vscode

smoke:
    cargo build -p apex-cli
    rm -rf /tmp/apex-smoke && mkdir -p /tmp/apex-smoke
    cd /tmp/apex-smoke && {{justfile_directory()}}/target/debug/apex init && {{justfile_directory()}}/target/debug/apex serve
    {{justfile_directory()}}/target/debug/apex scan {{justfile_directory()}}/test-fixtures/sample-repo > /tmp/apex-scan.json
    {{justfile_directory()}}/target/debug/apex languages | grep -q "Rust"
    {{justfile_directory()}}/target/debug/apex rules list | grep -q "RULE-LAYER-001"
    if {{justfile_directory()}}/target/debug/apex check {{justfile_directory()}}/test-fixtures/sample-repo > /tmp/apex-check.txt 2>&1; then echo "expected rule violation was not detected" >&2; exit 1; fi
    grep -q "RULE-LAYER-001" /tmp/apex-check.txt
    grep -q "UserService" /tmp/apex-scan.json
    npm run test:vscode

verify: fmt build test lint diagram smoke ui-smoke

clean-generated:
    rm -rf artifacts dist target node_modules .state/*_latest.txt
