#!/bin/bash

cd contracts
exit_code=0

for d in *; do
  cd $d  
  echo "------------------------------------"
  echo "Running test for $d contract"
  echo "------------------------------------"
  ls -la ../../target
  ls -la ../../target/wasm32-unknown-unknown/
  ls -la ../../target/wasm32-unknown-unknown/release
  ls -la ../../
  ls -la ../
  ls -la ./
  cargo make run_test
  if [ "$?" != 0 ]; then
    exit_code=$((1))
  fi
  cd ../
done
exit $exit_code
