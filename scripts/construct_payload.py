import json
import sys

try:
    with open("test_payload.html", "r", encoding="utf-8") as f:
        html = f.read()
    
    payload = {
        "html": html,
        "selector": "div.post-body a",
        "include_attributes": True,
        "max_results": 10000 
    }
    
    print(json.dumps(payload))
except Exception as e:
    sys.stderr.write(str(e))
    sys.exit(1)
