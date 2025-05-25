#!/usr/bin/env bash

set -ex

cargo build --release --target=x86_64-pc-windows-gnu
cargo build --release --target=i686-pc-windows-gnu

cp target/x86_64-pc-windows-gnu/release/ufb.dll ../Plugins/x86_64/ufb.dll
cp target/i686-pc-windows-gnu/release/ufb.dll ../Plugins/i686/ufb.dll
