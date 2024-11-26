#!/bin/bash

export RUSTFLAGS='-C target-cpu=native'

CARGO_TARGET_DIR=./baseline cargo build --release --features="baseline"
CARGO_TARGET_DIR=./fjall cargo build --release --features="fjall"
CARGO_TARGET_DIR=./sled cargo build --release --features="sled"
CARGO_TARGET_DIR=./libsql cargo build --release --features="libsql"
CARGO_TARGET_DIR=./sqlite cargo build --release --features="sqlite"
