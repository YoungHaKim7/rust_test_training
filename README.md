# rust_test_training

- `cargo t -- --nocapture && cargo nextest run --nocapture`

```bash
$ cargo t -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/rust_test_training-fbda1a55fd0ca8fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/add_test.rs (target/debug/deps/add_test-5460efe3594700bd)

running 1 test
input + input = 20
test add_fn ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rust_test_training

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


$ cargo nextest run --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
    Starting 1 test across 2 binaries
       START             rust_test_training::add_test add_fn

running 1 test
input + input = 20
test add_fn ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        PASS [   0.681s] rust_test_training::add_test add_fn
------------
     Summary [   0.682s] 1 test run: 1 passed, 0 skipped

```


# TEST
- https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests

- https://stackoverflow.com/questions/76979070/how-to-properly-use-a-tests-folder-in-a-rust-project

- https://github.com/kyclark/command-line-rust
