#!/bin/bash

# This script is a heavily modified version based on the scripts provided as part of the 2018/19 CS3102 Practical 1
# Sources
# https://unix.stackexchange.com/questions/418809/how-to-convert-an-input-parameter-to-integer-value-in-a-for-loop-in-bash (16/04/2020)
# https://opensource.com/article/18/5/you-dont-know-bash-intro-bash-arrays (16/04/2020)
# https://www.cyberciti.biz/faq/bash-for-loop/ (16/04/2020)
# https://stackoverflow.com/questions/6348902/how-can-i-add-numbers-in-a-bash-script (16/04/2020)

TEST_NUM=$1 # The test number/name.
SRC_OUTPUT_PATH=$2 # The base output file and path for the senders. Actual file name is {SRC_OUTPUT_PATH}_x for sender x.
RCV_OUTPUT_PATH=$3 # The base output file and path for the receivers. Actual file name is {RCV_OUTPUT_PATH}_x for receiver x.
SRC_TEST_INPUT=$4 # The base path for the expected input for the sender and receiver. The expected output is in the form of
RCV_TEST_INPUT=$5 # {SRC_TEST_INPUT/RCV_TEST_INPUT}_x for sender/receiver x.
SRC_EXPECTED_OUTPUT=$6 # The base path for the expected output for each sender. The expected output for sender x is in file {SRC_EXPECTED_OUTPUT}_x
RCV_EXPECTED_OUTPUT=$7 # The base path for the expected output for each receiver. The expected output for receiver x is in file {RCV_EXPECTED_OUTPUT}_x
KILL_WAIT=$8 # How long should the testing program wait before killing both receiver and sender forcefully (seconds).
RCV_COUNT=$9 # The number of receivers
SRC_COUNT=${10} # The number of senders

# The addresses of the machines used for the test. The addresses given here are host-names which are resolved by the the lab DNS.
REMOTE_PC=(pc3-017-l pc3-018-l pc3-019-l)

# The default ACN port.
PORT=5568

# the current directory;
CURRENT_DIR=$(pwd)

# Startup multiple remote receivers.
for i in $(seq "$RCV_COUNT")
do
    echo "Running rcv at ${REMOTE_PC[$((i - 1))]}"
    OUTPATH=${RCV_OUTPUT_PATH}'_'${i}'.temp'
    INPATH=${RCV_TEST_INPUT}'_'${i}
    ssh -n -f ${REMOTE_PC[$((i - 1))]} "sh -c 'cd ${CURRENT_DIR}; nohup ./rcv.sh > ${OUTPATH} < ${INPATH} 2>/dev/null'"
done

# Give the receivers a chance to startup.
sleep 2

# Startup multiple remote senders.
for i in $(seq "$SRC_COUNT")
do
    echo "Running src at ${REMOTE_PC[$((i - 1 + RCV_COUNT))]}"
    OUTPATH=${SRC_OUTPUT_PATH}'_'${i}'.temp'
    INPATH=${SRC_TEST_INPUT}'_'${i}
    INDEX=$(($i + $RCV_COUNT))
    ssh -n -f ${REMOTE_PC[$((i - 1 + RCV_COUNT))]} "sh -c 'cd ${CURRENT_DIR}; nohup ./src.sh > ${OUTPATH} < ${INPATH} 2>/dev/null'"
done

# Wait to allow all processes the chance to run, this has no specific guarantee that they will 
#	as it depends on scheduling but it is expected that this time period will be long enough that the code will have executed.
sleep ${KILL_WAIT}

# Kill programs running on all PC's
for pc in ${REMOTE_PC[@]}; do
    ssh ${pc} fuser -k -n udp ${PORT}
done

# Give time for all processes to be killed properly.
sleep 2
