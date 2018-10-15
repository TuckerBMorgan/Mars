perf record -g ./target/release/mars.*
sleep()
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg