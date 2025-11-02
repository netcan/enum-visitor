# enum-visitor

[![Crates.io](https://img.shields.io/crates/v/enum-visitor.svg)](https://crates.io/crates/enum-visitor)
[![docs.rs](https://docs.rs/enum-visitor/badge.svg)](https://docs.rs/enum-visitor)
[![CI](https://github.com/netcan/enum-visitor/actions/workflows/ci.yml/badge.svg)](https://github.com/netcan/enum-visitor/actions/workflows/ci.yml)

一个模仿 C++ `std::visit` 思想的轻量 Rust 库，通过宏与派生宏提供接近的使用体验。

英文为主文档（README.md）。

## 特性
- 通用宏 `visit!`：基于显式的枚举与变体列表生成 `match` 展开。
- 派生宏 `#[derive(VisitEnum)]`：在枚举所在模块生成局部宏，使你可以在 impl 内直接写 `visit!(self, |v| ...)`。

## 安装
在 Cargo.toml 中添加依赖：

```toml
[dependencies]
enum-visitor = "0.1"
```

派生用法（最贴近 `std::visit` 的书写）：

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
    fn area(&self) -> f64 { visit!(self, |s| s.area()) }
}
```

通用宏（无需 derive）：

```rust
enum_visitor::visit!(expr, Shape, [Circle, Rectangle, Triangle], |s| s.area());
```

运行仓库自带示例：`cargo run -p enum-visitor --example shapes`。

## 说明与限制
- 暂仅支持“单元素元组变体”（如 `Variant(T)`）。
- 派生会在枚举模块内生成两个宏：`visit_<enum_snake>!` 与局部 `visit!`。
  若同一模块对多个枚举同时派生，`visit!` 可能重名；请将枚举放入不同模块，
  或使用唯一名宏。

## 许可证
MIT（版权所有 © netcan）
