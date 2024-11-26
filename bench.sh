#!/bin/bash

hyperfine -N --warmup 5 baseline/release/index-data-test fjall/release/index-data-test sled/release/index-data-test libsql/release/index-data-test sqlite/release/index-data-test
