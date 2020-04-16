#!/bin/bash

# This script is based on the scripts provided as part of the 2018/19 CS3102 Practical 1

TEST_NUM=$1
SRC_OUTPUT_PATH=$2
RCV_OUTPUT_PATH=$3
SRC_TEST_INPUT=$4
RCV_TEST_INPUT=$5
SRC_EXPECTED_OUTPUT=$6
RCV_EXPECTED_OUTPUT=$7
KILL_WAIT=$8 # How long should the testing program wait before killing both receiver and sender forcefully (seconds).

# The default ACN port.
PORT=5568

# the current directory;
CURRENT_DIR=$(pwd)

# Check if the output matched the expected, diff will output only if they are different
# https://stackoverflow.com/questions/12137431/test-if-a-command-outputs-an-empty-string (09/02/2020)
if [[ $(diff -q -N ${SRC_OUTPUT_PATH} ${SRC_EXPECTED_OUTPUT}) ]];
then
  echo "Test ${TEST_NUM}: FAILED"
else
	if [[ $(diff -q -N ${RCV_OUTPUT_PATH} ${RCV_EXPECTED_OUTPUT}) ]];
	then
	  echo "Test ${TEST_NUM}: FAILED"
	else
	  echo "Test ${TEST_NUM}: PASSED"
	fi
fi
