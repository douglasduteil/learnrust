#!/bin/bash

set -exu

mkdir -p releases

for TARGET in "${@}"
do
  cargo build --target $TARGET --release --verbose
  tar Jcvf releases/learnrust-stable-$TARGET.tar.xz \
    -C target/$TARGET/release \
    learnrust
done

