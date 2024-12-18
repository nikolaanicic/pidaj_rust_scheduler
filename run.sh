#!/bin/bash
source ../../python/seminarski_venv/bin/activate
cargo build
mkdir -p results

function call_benchmark(){
	./target/debug/sem $1 $2 | python3 analyzer.py
}
export -f call_benchmark
client_calls=$(seq 50 50 300) 
retry_times=$(seq 1 10 100) 

parallel --group --tag --eta -- call_benchmark ::: $client_calls ::: $retry_times
