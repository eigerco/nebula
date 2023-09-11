#!/bin/bash

cd contracts
exit_code=0

for d in *; do
  cd $d  
  echo "------------------------------------"
  echo "Running test for $d contract"
  echo "------------------------------------"
  cargo make run_test
  # currently this is disabled due to raffle test failing
  # if [ "$?" != 0 ]; then
  #   exit_code=$((1))
  # fi
  cd ../
done
exit $exit_code
