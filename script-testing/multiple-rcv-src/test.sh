# Clean previous test output
rm -f test_out/*.temp

TEST_1_PATH='Test-1-Two-Data-Senders'
TEST_2_PATH='Test-2-Two-Data-Senders-Diff-Uni'

# Run the tests
sh test_run_multiple.sh 1-Two-Data-Senders test_out/test_1_src_out test_out/test_1_rcv_out ${TEST_1_PATH}/src ${TEST_1_PATH}/rcv ${TEST_1_PATH}/src_expected ${TEST_1_PATH}/rcv_expected 30 1 2
sh test_run_multiple.sh 2-Two-Data-Senders-Diff-Universes test_out/test_2_src_out test_out/test_2_rcv_out ${TEST_2_PATH}/src ${TEST_2_PATH}/rcv ${TEST_2_PATH}/src_expected ${TEST_2_PATH}/rcv_expected 30 1 2

# Run the checks to see if the tests passed. By waiting between running the test and checking it gives time for the file-system to syncronise the files.
# sh test_check.sh 1-Two-Data-Senders test_1_src_out test_1_rcv_out Test-1-Two-Data-Senders/src Test-1-Two-Data-Senders/rcv Test-1-Two-Data-Senders/src_expected Test-1-Two-Data-Senders/rcv_expected 5
