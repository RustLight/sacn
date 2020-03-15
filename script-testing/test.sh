# Clean previous test output
rm -f *.temp

# Run the tests
sh test_run.sh 0-Should-Fail 0src-out.temp 0rcv-out.temp test-0-should-fail/src test-0-should-fail/rcv test-0-should-fail/src_expected test-0-should-fail/rcv_expected 2
sh test_run.sh 1-Single-Uni-Data 1src-out.temp 1rcv-out.temp test-1-single-universe-data/test1_src test-1-single-universe-data/test1_rcv test-1-single-universe-data/src_expected test-1-single-universe-data/rcv_expected 2
sh test_run.sh 2-Two-Uni-Sync 2src-out.temp 2rcv-out.temp test-2-two-uni-sync/src test-2-two-uni-sync/rcv test-2-two-uni-sync/src_expected test-2-two-uni-sync/rcv_expected 2
sh test_run.sh 3-Discover-1-Src 3src-out.temp 3rcv-out.temp test-3-discover-1-src/src test-3-discover-1-src/rcv test-3-discover-1-src/src_expected test-3-discover-1-src/rcv_expected 20
sh test_run.sh 4-Overflow-Sequence 4src-out.temp 4rcv-out.temp test-4-single-universe-data-overflow-sequence/src test-4-single-universe-data-overflow-sequence/rcv test-4-single-universe-data-overflow-sequence/src_expected test-4-single-universe-data-overflow-sequence/rcv_expected 10
sh test_run.sh 5-terminate-universe 5src-out.temp 5rcv-out.temp test-5-terminate-universe/src test-5-terminate-universe/rcv test-5-terminate-universe/src_expected test-5-terminate-universe/rcv_expected 30

# Run the checks to see if the tests passed. By waiting between running the test and checking it gives time for the file-system to syncronise the files.
sh test_check.sh 0-Should-Fail 0src-out.temp 0rcv-out.temp test-0-should-fail/src test-0-should-fail/rcv test-0-should-fail/src_expected test-0-should-fail/rcv_expected 2
sh test_check.sh 1-Single-Uni-Data 1src-out.temp 1rcv-out.temp test-1-single-universe-data/test1_src test-1-single-universe-data/test1_rcv test-1-single-universe-data/src_expected test-1-single-universe-data/rcv_expected 2
sh test_check.sh 2-Two-Uni-Sync 2src-out.temp 2rcv-out.temp test-2-two-uni-sync/src test-2-two-uni-sync/rcv test-2-two-uni-sync/src_expected test-2-two-uni-sync/rcv_expected 2
sh test_check.sh 3-Discover-1-Src 3src-out.temp 3rcv-out.temp test-3-discover-1-src/src test-3-discover-1-src/rcv test-3-discover-1-src/src_expected test-3-discover-1-src/rcv_expected 20
sh test_check.sh 4-Overflow-Sequence 4src-out.temp 4rcv-out.temp test-4-single-universe-data-overflow-sequence/src test-4-single-universe-data-overflow-sequence/rcv test-4-single-universe-data-overflow-sequence/src_expected test-4-single-universe-data-overflow-sequence/rcv_expected 10
sh test_check.sh 5-terminate-universe 5src-out.temp 5rcv-out.temp test-5-terminate-universe/src test-5-terminate-universe/rcv test-5-terminate-universe/src_expected test-5-terminate-universe/rcv_expected 30
