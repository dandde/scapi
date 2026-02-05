#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== 1. Auto-Build ===${NC}"
cargo build

echo -e "${GREEN}=== 2. Auto-Test ===${NC}"
cargo test

echo -e "${GREEN}=== 3. Auto-Init Server ===${NC}"
# Kill any existing process on port 3000 to avoid conflicts (naive check)
lsof -ti:3000 | xargs kill -9 2>/dev/null || true

# Start server in background
cargo run &
SERVER_PID=$!

# Ensure we kill the server on exit
trap "kill $SERVER_PID" EXIT

echo "Waiting for server to start on port 3000..."
# Loop wait for port 3000
for i in {1..30}; do
    if nc -z 127.0.0.1 3000 2>/dev/null; then
        echo "Server is up!"
        break
    fi
    sleep 0.5
    echo -n "."
done

echo ""

echo -e "${GREEN}=== 4. Auto-Connect ===${NC}"
echo "Sending test request to /api/v1/fetch..."

# Test fetch endpoint with example.com
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST http://127.0.0.1:3000/api/v1/fetch \
     -H "Content-Type: application/json" \
     -d '{"url": "https://palimyanmarpitaka.blogspot.com/"}')

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Code: $HTTP_CODE"
# Echo first 50 chars of body to show connectivity without spamming
echo "Response Body Preview: ${BODY:0:100}..."

if [[ "$HTTP_CODE" == "200" ]]; then
    echo -e "${GREEN}SUCCESS: Connected and fetched successfully!${NC}"
else
    echo "FAILURE: Server returned error code $HTTP_CODE"
    exit 1
fi

wait $SERVER_PID
