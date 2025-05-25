#!/usr/bin/env bash

set -ex

cargo build --release --target=x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/libufb.so ../Plugins/x86_64/libufb.so
