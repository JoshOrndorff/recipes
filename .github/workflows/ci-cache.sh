#!/bin/bash

set -eux

HEAD_REF=$1
TOOLCHAIN=$2

CARGO_HOME=/cache/$HEAD_REF/$TOOLCHAIN
CARGO_TARGET_DIR=/cache/$HEAD_REF/$TOOLCHAIN
SCCACHE_DIR=/cache/$HEAD_REF/$TOOLCHAIN/sccache

echo "::set-env name=CARGO_HOME::$CARGO_HOME"
echo "::set-env name=CARGO_TARGET_DIR::$CARGO_TARGET_DIR"
echo "::set-env name=SCCACHE_DIR::$SCCACHE_DIR"

mkdir -p $CARGO_TARGET_DIR;

echo "Current Rust nightly version:";
rustc +nightly --version;
echo "Cached Rust nightly version:";

if [ ! -f $CARGO_TARGET_DIR/check_nightly_rust ]; then
    echo "" > $CARGO_TARGET_DIR/check_nightly_rust;
fi

cat $CARGO_TARGET_DIR/check_nightly_rust;

if [[ $(cat $CARGO_TARGET_DIR/check_nightly_rust) == $(rustc +nightly --version) ]];
    then echo "The Rust nightly version has not changed";
else
    echo "The Rust nightly version has changed. Clearing the cache";
    rm -rf $CARGO_TARGET_DIR/*;
fi
