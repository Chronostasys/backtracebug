# Backtrace bug while use lld for linking

Link backtrace function using lld, and run the program, it will crash(Segmentation fault).  

Run following command on macos should reproduce it. Not sure if it occurs on other platforms.

```bash
cargo build
clang test.c target/debug/libvm.a  -v -fuse-ld=lld
./a.out
```

