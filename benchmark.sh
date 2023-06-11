#!/usr/bin/env bash

# install the `cpuset` package to use these benchmarks
# also set systemd.unified_cgroup_hierarchy=false in your kernel parameters

set -e

echo "compiling benchmarks"
cargo bench --no-run

echo "setting up system for benchmarks"

fixup() {
  echo "cleaning up"
  sudo cset shield --reset
  sudo sh -c "echo 1 > /sys/devices/system/cpu/cpufreq/boost"
}

sudo sh -c "echo 0 > /sys/devices/system/cpu/cpufreq/boost"
sudo cset shield -k on -c 0
trap "fixup" ERR

echo "finding benchmark binary"
BENCHMARK_BIN=$(find target/release/deps -name "*bench*" | grep -P -v ".*\.d")

echo "benchmarking"
sudo cset shield --exec -- nice -n -5 "$BENCHMARK_BIN" --bench

fixup


