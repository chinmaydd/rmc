#!/usr/bin/env bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

# Test for platform
platform=`uname -sp`
if [[ $platform != "Linux x86_64" ]]; then
  echo "Codegen script only works on Linux x86 platform"
  exit 0
fi

# Get RMC root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
RMC_DIR=$SCRIPT_DIR/..

# Log output
STD_LIB_LOG="/tmp/StdLibTest/log.txt"
echo "Starting RMC codegen for the Rust standard library"
cd /tmp
if [ -d StdLibTest ]; then rm -rf StdLibTest; fi
cargo new StdLibTest
cd StdLibTest

# Check that we have the nighly toolchain, which is required for -Z build-std
if ! rustup toolchain list | grep -q nightly; then
  echo "Installing nightly toolchain"
  rustup toolchain install nightly
fi
RUSTFLAGS="-Z trim-diagnostic-paths=no -Z codegen-backend=gotoc --cfg=rmc" RUSTC=rmc-rustc cargo +nightly build -Z build-std --target x86_64-unknown-linux-gnu 2> $STD_LIB_LOG

# For now, we expect a linker error, but no modules should fail with a compiler
# panic. 
#
# With https://github.com/model-checking/rmc/issues/109, this check can be
# removed to just allow the success of the previous line to determine the 
# success of this script (with no $STD_LIB_LOG needed)
RESULT=$?
if grep -q "error: extern location for std does not exist:" $STD_LIB_LOG 
then
  echo "Successful RMC codegen for the Rust standard library."
  echo "Expected linking failure: https://github.com/model-checking/rmc/issues/109,"
elif $RESULT -eq 0
then
  echo "Successful RMC codegen for the Rust standard library."
else 
  echo "Unexpected panic on building standard library"
  cat $STD_LIB_LOG
  exit 1
fi
