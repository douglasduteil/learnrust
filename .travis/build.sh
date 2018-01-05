#!/bin/bash

set -exu pipefail $@

mkdir -p releases

for TARGET in "${@:2}"
do
  cargo build --target $TARGET --release --verbose
  tar Jcvf releases/learnrust-stable-$TARGET.tar.xz target/$TARGET/release/learnrust
done

