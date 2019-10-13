# Benchmarking Logging Libraries for Rust

## Running Tests

Rust has a problem when you try to "reset" logging twice in a runtime,
it appears.

You must run the tests individually for it to work:

```
cargo bench --color=always --package logging-tests --bin logging-tests tests::bench_fern_single_filtered_line -- --exact
```

Using an IDE like CLion will greatly help with running the tests where it
becomes a 1-click operation.

