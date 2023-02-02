cargo build
clang test.c target/debug/libtest.a  -v -fuse-ld=lld
./a.out
