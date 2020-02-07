#!/bin/bash

# This script is based on the scripts provided as part of the 2018/19 CS3102 Practical 1

SRC_OUTPUT_PATH=$1
RCV_OUTPUT_PATH=$2
SRC_TEST_INPUT=$3
RCV_TEST_INPUT=$4


# host where server is running
REMOTE_PC=pc5-007-l
REMOTE_PC_2=pc5-013-l

PORT=5568

SERVER_HOST=klovia.cs.st-andrews.ac.uk

# the current directory;
CURRENT_DIR=$(pwd)

# i) Startup a sender on the first remote pc
echo "Running src at ${REMOTE_PC}"
ssh -n -f ${REMOTE_PC} "sh -c 'cd ${CURRENT_DIR}; nohup ./src.sh > ${SRC_OUTPUT_PATH} < ${SRC_TEST_INPUT}"

# ii) Startup a receiver on the second remote pc
echo "Running rcv at ${REMOTE_PC_2}"
ssh -n -f ${REMOTE_PC_2} "sh -c 'cd ${CURRENT_DIR}; nohup ./rcv.sh > ${RCV_OUTPUT_PATH} < ${RCV_TEST_INPUT}"

# iii) Wait to allow both processes the chance to run, this has no specific guarantee that they will 
#	as it depends on scheduling but it is hoped this will be long enough.
sleep 10

# Kill stuff running on both PC's
ssh ${REMOTE_PC} fuser -k -n udp ${PORT}
ssh ${REMOTE_PC_2} fuser -k -n udp ${PORT}
