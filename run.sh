#!/bin/bash
source ../../python/seminarski_venv/bin/activate
cargo build
mkdir -p results

function call_benchmark(){
	cargo run -r $1 $2 | python3 analyzer.py
}
export -f call_benchmark
client_calls=(5 10 15 20 25 30 50)
retries=(1 5 10)


parallel --group --tag --eta 'call_benchmark {}' ::: "${client_calls[@]}" ::: "${retries[@]}"
