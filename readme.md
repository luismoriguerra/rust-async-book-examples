Book examples
1. Introduction to Async | Async Rust
https://learning.oreilly.com/library/view/async-rust/9781098149086/ch01.html#id45



example project 
https://github.com/jeremychone-channel/rust-xp-ollama/blob/main/src/lib.rs

examples folder 
https://doc.rust-lang.org/cargo/guide/project-layout.html

.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
