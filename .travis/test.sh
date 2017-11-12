#!/bin/bash

set -exu pipefail

cd workshopper
cargo test

cd -
cargo test
