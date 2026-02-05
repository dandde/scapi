#!/bin/bash
curl -X POST http://localhost:3000/select \
-H "Content-Type: application/json" \
-d '{"html": "<div><p>Hello</p></div>", "selector": "p"}'
