#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== SCAPI Benchmark Runner ===${NC}"
echo "Compiling benchmark in release mode for accurate timing..."

# Build and run the benchmark example in release mode
cargo run --release --example benchmark_fetch

echo -e "\n${GREEN}Benchmark complete.${NC}"
