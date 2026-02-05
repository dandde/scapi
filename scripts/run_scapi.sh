#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== 1. Auto-Build ===${NC}"
cargo build

echo -e "${GREEN}=== 3. Auto-Init Server ===${NC}"
# Kill any existing process on port 3000 to avoid conflicts (naive check)
lsof -ti:3000 | xargs kill -9 2>/dev/null || true

# Start server in background
cargo run --release --bin scapi &
SERVER_PID=$!
echo "Server started with PID $SERVER_PID"
# sleep 5 # Wait for server to start