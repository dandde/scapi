#!/bin/bash
set -e

# Configuration
URL="https://palimyanmarpitaka.blogspot.com/2021/04/blog-post.html"
SELECTOR="div.post-body a"
HTML_FILE="test_payload.html"
API_URL="http://localhost:3000/api/v1/select"

echo "=== SCAPI Select API Test ==="
echo "Target URL: $URL"
echo "Selector:   $SELECTOR"

# 1. Fetch HTML if not exists
if [ ! -f "$HTML_FILE" ]; then
    echo "Fetching HTML..."
    curl -s -L "$URL" -o "$HTML_FILE"
    echo "Fetched $(du -h $HTML_FILE | cut -f1)"
else
    echo "Using cached HTML file ($(du -h $HTML_FILE | cut -f1))"
fi

# 2. Construct Payload using Python (safe escaping)
echo "Constructing JSON payload..."
cat <<EOF > construct_payload.py
import json
import sys

try:
    with open("$HTML_FILE", "r", encoding="utf-8") as f:
        html = f.read()
    
    payload = {
        "html": html,
        "selector": "$SELECTOR",
        "include_attributes": True,
        "max_results": 10000 
    }
    
    print(json.dumps(payload))
except Exception as e:
    sys.stderr.write(str(e))
    sys.exit(1)
EOF

# 3. Send Request
echo "Sending POST request to $API_URL..."
start_time=$(date +%s%N)
# We pipe python output directly to curl to avoid shell variable limits on some systems
python3 construct_payload.py | curl -s -X POST "$API_URL" \
    -H "Content-Type: application/json" \
    -d @- > response.json
end_time=$(date +%s%N)

# 4. Analyze Response
if [ $? -eq 0 ]; then
    duration=$(( ($end_time - $start_time) / 1000000 ))
    echo "Request completed in ${duration}ms"
    
    # Check for success
    if grep -q '"error":' response.json; then
        echo "API returned error:"
        cat response.json | python3 -c "import sys, json; print(json.load(sys.stdin).get('details', 'Unknown error'))"
    else
        echo "Success!"
        # Extract count and first match using python
        python3 -c "
import sys, json
data = json.load(open('response.json'))
print(f\"Total Matches: {data.get('count', 'N/A')}\")
print(f\"Selector Type: {data.get('selector_type', 'N/A')}\")
matches = data.get('matches', [])
if matches:
    print(f\"First Match Tag: {matches[0].get('tag')}\")
    print(f\"First Match Text: {matches[0].get('text', '')[:50]}...\")
    print(f\"First Match Attrs: {matches[0].get('attributes')}\")
"
    fi
else
    echo "Curl failed."
fi

# Cleanup
rm construct_payload.py
# rm response.json # Keep for inspection
echo "============================="
