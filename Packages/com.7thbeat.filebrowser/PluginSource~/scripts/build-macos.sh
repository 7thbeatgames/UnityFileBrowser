#!/usr/bin/env bash

set -ex

targets=(x86_64-apple-darwin aarch64-apple-darwin)

for target in "${targets[@]}" ; do
    cargo build --release --target="$target"
done

args=()

for target in "${targets[@]}" ; do
    args+=("target/${target}/release/libufb.dylib");
done

lipo -create "${args[@]}" -output ../Plugins/ufb.dylib
