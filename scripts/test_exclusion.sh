#!/bin/bash
set -e

# Configuration
URL="https://palimyanmarpitaka.blogspot.com/2021/04/blog-post.html"
SELECTOR="body div.post-body"
EXCLUDE="[\"head\", \"a[href='#']\", \"a[href='']\"]" # Exclude empty or hash links
HTML_FILE="test_payload.html"
API_URL="http://localhost:3000/api/v1/select"

echo "=== SCAPI Selector Exclusion Test ==="

# 1. Fetch HTML if not exists
if [ ! -f "$HTML_FILE" ]; then
    echo "Fetching HTML..."
    curl -s -L "$URL" -o "$HTML_FILE"
    echo "Fetched $(du -h $HTML_FILE | cut -f1)"
else
    echo "Using cached HTML file ($(du -h $HTML_FILE | cut -f1))"
fi

# 2. Construct Payload (With Exclusions)
echo "Constructing JSON payload..."
cat <<EOF > construct_payload_exclude.py
import json
import sys

try:
    with open("$HTML_FILE", "r", encoding="utf-8") as f:
        html = f.read()
    
    payload = {
        "html": html,
        "selector": "$SELECTOR",
        "exclude_selectors": $EXCLUDE,
        "include_attributes": True,
        "include_html": True,
        "max_results": 10000 
    }
    
    print(json.dumps(payload))
except Exception as e:
    sys.stderr.write(str(e))
    sys.exit(1)
EOF

# 3. Send Request
echo "Sending POST request (with exclusions) to $API_URL..."
python3 construct_payload_exclude.py | curl -s -X POST "$API_URL" \
    -H "Content-Type: application/json" \
    -d @- > response_exclude.json

# 4. Analyze Results
echo "Comparing results..."
python3 -c "
import json
try:
    data = json.load(open('response_exclude.json'))
    count = data.get('count', 'N/A')
    print(f'Count with exclusions: {count}')
    
    # Simple check: verify typical exclude targets are gone (heuristic)
    matches = data.get('matches', [])
    bad_links = [m for m in matches if m['attributes'].get('href') in ['#', '']]
    print(f'Bad links remaining: {len(bad_links)}')
except Exception as e:
    print(f'Error analyzing: {e}')
"

# Cleanup
rm construct_payload_exclude.py
echo "============================="
