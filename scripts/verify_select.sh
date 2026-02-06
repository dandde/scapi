#!/bin/bash

# Kill any running scapi instance
pkill -f "scapi" || true

# Start server in background
echo "Starting SCAPI server..."
cargo run --bin scapi > server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
sleep 10

echo "Running verification tests..."

# Test 1: Buffered Select (HTMLer) - Small Payload
echo "Test 1: /api/v1/select (Small Payload)"
curl -v -X POST http://localhost:3000/api/v1/select \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><div class=\"content\">Hello World</div></body></html>",
    "selector": "div.content"
  }' > response_select.json 2>&1

cat response_select.json
echo -e "\n"

# Test 2: Forced Streaming Select (lol_html)
echo "Test 2: /api/v1/select-stream"
curl -v -X POST http://localhost:3000/api/v1/select-stream \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><div class=\"stream\">Streaming Match</div></body></html>",
    "selector": "div.stream"
  }' > response_stream.json 2>&1

cat response_stream.json
echo -e "\n"

# Cleanup
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null
