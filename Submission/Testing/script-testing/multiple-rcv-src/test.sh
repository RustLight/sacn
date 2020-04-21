# Clean previous test output
rm -f test_out/*.temp

TEST_1_PATH='Test-1-Two-Data-Senders'
TEST_2_PATH='Test-2-Two-Data-Senders-Diff-Uni'
TEST_3_PATH='Test-3-One-Sender-Two-Receiver'
TEST_4_PATH='Test-4-One-Src-Two-Distinct-Rcv'
TEST_5_PATH='Test-5-Three-Rcv-Two-Src'

# Run the tests
sh test_run_multiple.sh 1-Two-Data-Senders test_out/test_1_src_out test_out/test_1_rcv_out ${TEST_1_PATH}/src ${TEST_1_PATH}/rcv ${TEST_1_PATH}/src_expected ${TEST_1_PATH}/rcv_expected 30 1 2
sh test_run_multiple.sh 2-Two-Data-Senders-Diff-Universes test_out/test_2_src_out test_out/test_2_rcv_out ${TEST_2_PATH}/src ${TEST_2_PATH}/rcv ${TEST_2_PATH}/src_expected ${TEST_2_PATH}/rcv_expected 30 1 2
sh test_run_multiple.sh 3-Two-Rcv-One-Src test_out/test_3_src_out test_out/test_3_rcv_out ${TEST_3_PATH}/src ${TEST_3_PATH}/rcv ${TEST_3_PATH}/src_expected ${TEST_3_PATH}/rcv_expected 30 2 1
sh test_run_multiple.sh 4-Two-Distinct-Rcv-One-Src test_out/test_4_src_out test_out/test_4_rcv_out ${TEST_4_PATH}/src ${TEST_4_PATH}/rcv ${TEST_4_PATH}/src_expected ${TEST_4_PATH}/rcv_expected 30 2 1
sh test_run_multiple.sh 5-Three-Rcv-Two-Src test_out/test_5_src_out test_out/test_5_rcv_out ${TEST_5_PATH}/src ${TEST_5_PATH}/rcv ${TEST_5_PATH}/src_expected ${TEST_5_PATH}/rcv_expected 30 3 2

# Output has multiple correct possibilities due to the nature of multiple concurrent machines running so therefore the automatic checks cannot be performed under the current system.
# See expected-results.pdf