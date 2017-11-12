#!/bin/bash

set -exu pipefail

cargo build --target $TARGET
cargo build --target $TARGET --release
