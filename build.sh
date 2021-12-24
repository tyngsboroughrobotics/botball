#!/bin/bash

set -e

(cd lib && cargo build --release)

cp lib/target/release/libbotball.dylib ./libbotball || true
cp lib/target/release/libbotball.so ./libbotball || true
