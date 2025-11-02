# enum-visitor

[![Crates.io](https://img.shields.io/crates/v/enum-visitor.svg)](https://crates.io/crates/enum-visitor)
[![docs.rs](https://docs.rs/enum-visitor/badge.svg)](https://docs.rs/enum-visitor)
[![CI](https://github.com/netcan/enum-visitor/actions/workflows/ci.yml/badge.svg)](https://github.com/netcan/enum-visitor/actions/workflows/ci.yml)

Tiny helper to emulate a C++ `std::visit`-like experience in Rust using macros and a proc-macro derive.

## Features
- Universal macro `visit!` that expands to a `match` over explicit variants.
- Derive `#[derive(VisitEnum)]` which generates a local `visit!` and a unique `visit_<enum>!` in the enum’s module.

## Install
```toml
[dependencies]
enum-visitor = "0.1"
```

## Example (derive)
```rust
#[derive(enum_visitor::VisitEnum)]
enum Shape { Circle(f64), Rect((f64,f64)) }

impl Shape {
    fn area(&self) -> f64 {
        visit!(self, |s| match s {
            r @ f64::NAN => r, // just to show arbitrary use
            _ => 0.0,
        })
    }
}
```

See repository README for a full example with structs and a triangle.
