set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    just --list

setup:
    npm install --no-audit --no-fund

fmt:
    cargo fmt --all
    gofmt -w collab/*.go

build:
    cargo build --workspace
    npm run build

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
    curl -fsSL "http://127.0.0.1:4322/api/diagram?path={{justfile_directory()}}/test-fixtures/sample-repo&format=svg" | grep -q "<svg"
    curl -fsSL http://127.0.0.1:4322/ | grep -q "Apex Workbench"

vscode-smoke:
    cargo build -p apex-cli
    npm run test:vscode

smoke:
    cargo build -p apex-cli
    rm -rf /tmp/apex-smoke && mkdir -p /tmp/apex-smoke
    cd /tmp/apex-smoke && {{justfile_directory()}}/target/debug/apex init && {{justfile_directory()}}/target/debug/apex serve
    {{justfile_directory()}}/target/debug/apex scan {{justfile_directory()}}/test-fixtures/sample-repo > /tmp/apex-scan.json
    if {{justfile_directory()}}/target/debug/apex check {{justfile_directory()}}/test-fixtures/sample-repo > /tmp/apex-check.txt 2>&1; then echo "expected rule violation was not detected" >&2; exit 1; fi
    grep -q "RULE-LAYER-001" /tmp/apex-check.txt
    grep -q "UserService" /tmp/apex-scan.json
    npm run test:vscode

verify: fmt build test lint diagram smoke ui-smoke

clean-generated:
    rm -rf artifacts dist target node_modules docs/*_latest.txt
