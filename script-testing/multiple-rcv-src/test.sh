# Clean previous test output
rm -f *.temp

# Run the tests
sh test_run_multiple.sh 1-Two-Data-Senders test_1_src_out test_1_rcv_out Test-1-Two-Data-Senders/src Test-1-Two-Data-Senders/rcv Test-1-Two-Data-Senders/src_expected Test-1-Two-Data-Senders/rcv_expected 10 1 2

# Run the checks to see if the tests passed. By waiting between running the test and checking it gives time for the file-system to syncronise the files.
# sh test_check.sh 1-Two-Data-Senders test_1_src_out test_1_rcv_out Test-1-Two-Data-Senders/src Test-1-Two-Data-Senders/rcv Test-1-Two-Data-Senders/src_expected Test-1-Two-Data-Senders/rcv_expected 5
