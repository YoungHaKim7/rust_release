# Test


```bash
$ cargo t -- --nocapture
   Compiling a01_rust_unit_tests v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/111_rust/11111_oj/rust_release/0002_cargo_test_rustc/a01_rust_unit_tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/a01_rust_unit_tests-514c9fd6fbd11599)

running 1 test
"/etc/hosts"
##
# Host Database
#
# localhost is used to configure the loopback interface
# when the system is booting.  Do not change this entry.
##
127.0.0.1	localhost
255.255.255.255	broadcasthost
::1             localhost

test tests::test_read_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests a01_rust_unit_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# rustc를 이용한 테스트 방법

```bash
$ rustc --test lib.rs

$ ./lib --nocapture

running 1 test
"/etc/hosts"
##
# Host Database
#
# localhost is used to configure the loopback interface
# when the system is booting.  Do not change this entry.
##
127.0.0.1	localhost
255.255.255.255	broadcasthost
::1             localhost

test tests::test_read_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
