#!/bin/bash

# Run the demo_src, this script is based on the scripts provided as part of the 2018/19 CS3102 Practical 1

# host where server is running
REMOTE_PC=pc5-007-l
REMOTE_PC_2=pc5-013-l

PORT=5568

SERVER_HOST=klovia.cs.st-andrews.ac.uk

# the current directory;
CURRENT_DIR=$(pwd)

# i) login and run server in the background
echo "Running src at ${REMOTE_PC}"
ssh -n -f ${REMOTE_PC} "sh -c 'cd ${CURRENT_DIR}; nohup ./src.sh > /dev/null 2>&1'"

# ii) run client playing back
echo "Running rcv at ${REMOTE_PC_2}"
ssh -n -f ${REMOTE_PC_2} "sh -c 'cd ${CURRENT_DIR}; nohup ./rcv.sh > /dev/null 2>&1 &'"

# Kill stuff running on both PC's
ssh ${REMOTE_PC} fuser -k -n udp ${PORT}
ssh ${REMOTE_PC_2} fuser -k -n udp ${PORT}
