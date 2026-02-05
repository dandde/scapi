import json
import re

try:
    with open('response_exclude.json', 'r') as f:
        data = json.load(f)
        
    print(f"Total Matches: {data.get('count')}")
    matches = data.get('matches', [])
    
    for i, m in enumerate(matches):
        print(f"--- Match {i+1}: {m['tag']} ---")
        html = m.get('html', '')
        print(f"HTML Snippet: {html[:100]}...")
        
        # Check for excluded links in the HTML content
        if "href='#'" in html or 'href="#"' in html:
            print("WARNING: Found excluded link (href='#') inside content!")
        else:
            print("No href='#' found in snippet (might be deeper).")
            # Deeper check
            if re.search(r'href=["\']#["\']', html):
                 print("WARNING: Regex found href='#' in content!")
            else:
                 print("Clean of href='#'")

except Exception as e:
    print(f"Error: {e}")
