#!/usr/bin/env bash

cargo build --release --target=x86_64-unknown-linux-gnu || exit 1
cp target/x86_64-unknown-linux-gnu/release/libunityfiledialog.so ../Plugins/x86_64/libunityfiledialog.so || exit 1
