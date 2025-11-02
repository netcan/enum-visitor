# enum-visitor

一个模仿 C++ `std::visit` 思想的轻量 Rust 库，提供：

- 通用宏 `visit_with!`：基于枚举/变体列表生成 `match` 展开。
- 派生宏 `#[derive(VisitEnum)]`：为具体枚举生成专用宏 `visit_with_<enum>!`，
  以便只写一次闭包体便可匹配所有变体。

注意：由于 Rust 目前缺乏对“泛型闭包”的直接支持，想要像 C++ `std::visit` 那样
不指明枚举类型就自动展开所有变体在稳定版 Rust 中不可行。本库提供两种务实方案：

1) 通用宏（无需 derive）：

```rust
visit_with!(expr, Shape, [Circle, Rectangle, Triangle], |s| s.area())
```

2) 专用宏（需 derive，最贴近题述“`visit_with!(self, |s| s.area())`”的写法）：

```rust
#[derive(enum_visitor::VisitEnum)]
enum Shape { Circle(Circle), Rectangle(Rectangle), Triangle(Triangle) }

impl Shape {
    fn area(&self) -> f64 {
        // 派生宏会在此模块内注入一个局部宏 `visit_with!`
        visit_with!(self, |s| s.area())
    }
}
```

两者都会在编译期展开为等价的 `match`：

```rust
match self {
    Shape::Circle(s) => s.area(),
    Shape::Rectangle(s) => s.area(),
    Shape::Triangle(s) => s.area(),
}
```

参见 `enum-visitor/examples/shapes.rs` 获取完整示例。

许可：MIT（版权归 netcan 所有）。
