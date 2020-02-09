sh testing.sh 0-Should-Fail src-out.temp rcv-out.temp test-0-should-fail/src test-0-should-fail/rcv test-0-should-fail/src_expected test-0-should-fail/rcv_expected 2
sh testing.sh 1-Single-Uni-Data src-out.temp rcv-out.temp test-1-single-universe-data/test1_src test-1-single-universe-data/test1_rcv test-1-single-universe-data/src_expected test-1-single-universe-data/rcv_expected 2
sh testing.sh 2-Two-Uni-Sync 2src-out.temp 2rcv-out.temp test-2-two-uni-sync/src test-2-two-uni-sync/rcv test-2-two-uni-sync/src_expected test-2-two-uni-sync/rcv_expected 2
sh testing.sh 3-Discover-1-Src 3src-out.temp 3rcv-out.temp test-3-discover-1-src/src test-3-discover-1-src/rcv test-3-discover-1-src/src_expected test-3-discover-1-src/rcv_expected 20
