#!/bin/bash

set -exu pipefail

cargo build --target $TARGET --verbose
cargo build --target $TARGET --release --verbose
