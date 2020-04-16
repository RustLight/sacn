#!/bin/bash

SRC_NAME=$1 # The source name

# Run the demo_src, this script is based on the scripts provided as part of the 2018/19 CS3102 Practical 1
cargo -q run --bin demo_src 0.0.0.0 ${SRC_NAME}
