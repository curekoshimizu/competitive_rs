#!/bin/bash

set -eu

find . -name "Cargo.toml" | while read -r path
do
  TARGET_DIR=`dirname "$path"`
  pushd $TARGET_DIR > /dev/null
  echo $PWD
  cargo build
  cargo test
  popd > /dev/null
done
exit $?
