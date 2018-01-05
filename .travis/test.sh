#!/bin/bash

set -exu pipefail

cd workshopper
cargo test --verbose

cd -
cargo test --verbose
