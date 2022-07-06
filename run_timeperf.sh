#!/bin/bash

set -eux

which rustup
which cargo

pushd "./timeperf-original"
cargo run --release -- -d ../resources/system.dic -s ../wagahaiwa_nekodearu.txt -r ../resources
popd

pushd "./timeperf-packed"
cargo run --release -- -d ../resources/system.dic -s ../wagahaiwa_nekodearu.txt -r ../resources
popd
