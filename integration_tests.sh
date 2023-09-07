#!/bin/bash

cd contracts
exit_code=0

{ ERR=$( { echo "sth" 1>&3 ; } 2>&1); } 3>&1
if [ ! -z "$ERR" ]; then
  echo "$ERR"
  exit 1
fi

for d in *; do
  cd $d  
  echo "------------------------------------"
  echo "Running test for $d contract"
  echo "------------------------------------"
  cargo make run_test
  if [ "$?" != 0 ]; then
    exit_code=$((1))
  fi
  cd ../
done
exit $exit_code
