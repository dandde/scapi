import urllib.request
import time
import json
import ssl

def verify_streaming():
    url = "http://localhost:3000/api/v1/fetch"
    target_url = "https://httpbin.org/drip?duration=2&numbytes=10&code=200&delay=0"
    
    payload = {
        "url": target_url,
        "timeout_ms": 10000
    }
    
    data = json.dumps(payload).encode('utf-8')
    req = urllib.request.Request(url, data=data, headers={
        'Content-Type': 'application/json',
        'User-Agent': 'SCAPI-Verifier'
    })
    
    print(f"Sending request to {url} targeting {target_url}")
    
    try:
        start_time = time.time()
        # open url
        with urllib.request.urlopen(req) as response:
            print(f"Response status: {response.status}")
            print(f"Headers: {response.headers}")
            
            if response.status != 200:
                print("Error: Status not 200")
                return

            ttfb = time.time() - start_time
            print(f"Time to First Byte (TTFB): {ttfb:.4f}s")
            
            chunk_count = 0
            byte_count = 0
            
            while True:
                chunk = response.read(1) # Read 1 byte at a time to detect streaming
                if not chunk:
                    break
                
                chunk_count += 1
                byte_count += len(chunk)
                current_time = time.time() - start_time
                
                # Print first few chunks to verify timing
                if chunk_count <= 5:
                     print(f"Received byte {chunk_count}: {chunk} at {current_time:.4f}s")
            
            total_time = time.time() - start_time
            print(f"Total chunks/bytes read loop iterations: {chunk_count}")
            print(f"Total bytes: {byte_count}")
            print(f"Total time: {total_time:.4f}s")
            
            # If TTFB is significantly smaller than Total Time, it indicates streaming
            if total_time > ttfb + 0.5:
                print("SUCCESS: Streaming detected (Total time > TTFB)")
            else:
                print("WARNING: Streaming might not be working (Total time close to TTFB)")
                
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    verify_streaming()
