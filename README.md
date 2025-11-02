# enum-visitor

A tiny Rust library that emulates a C++ `std::visit`-like experience using macros and a proc-macro derive.

English is the primary README. 中文版请见 README.zh.md.

## Features
- Universal macro `visit_with!` that expands to a `match` over explicit variants.
- Derive `#[derive(VisitEnum)]` to generate a local macro so you can simply write `visit_with!(self, |v| ...)` inside the enum’s module/impl.

## Quick Start
Add dependency (path example while unpublished):

```toml
[dependencies]
enum-visitor = { path = "./enum-visitor" }
```

Derive-based usage (closest to C++ `std::visit` ergonomics):

```rust
use std::f64::consts::PI;

struct Circle { radius: f64 }
impl Circle { fn area(&self) -> f64 { PI * self.radius * self.radius } }
struct Rectangle { width: f64, height: f64 }
impl Rectangle { fn area(&self) -> f64 { self.width * self.height } }
struct Triangle { base: f64, height: f64 }
impl Triangle { fn area(&self) -> f64 { 0.5 * self.base * self.height } }

#[derive(enum_visitor::VisitEnum)]
enum Shape { Circle(Circle), Rectangle(Rectangle), Triangle(Triangle) }

impl Shape {
    fn area(&self) -> f64 { visit_with!(self, |s| s.area()) }
}
```

Universal macro (no derive):

```rust
enum_visitor::visit_with!(expr, Shape, [Circle, Rectangle, Triangle], |s| s.area());
```

Run the example: `cargo run -p enum-visitor --example shapes`.

## Notes & Limitations
- Supported variants: tuple variants with exactly one field (e.g., `Variant(T)`).
- The derive creates two macros in the enum’s module: `visit_with_<enum_snake>!` and a local `visit_with!`. To avoid name clashes, place different enums in separate modules or use the unique macro name.

## License
MIT © netcan
