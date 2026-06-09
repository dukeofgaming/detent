#!/usr/bin/env bash

set -euo pipefail

cd /workspaces/detent

server_pattern="python3 -m http.server 8080 -d target/cucumber-report"

if pgrep -f "$server_pattern" >/dev/null 2>&1; then
    echo "Cucumber report already running at http://localhost:8080"
    exit 0
fi

echo "Cucumber report available at http://localhost:8080"
python3 -m http.server 8080 -d target/cucumber-report