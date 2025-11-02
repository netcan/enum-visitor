use std::f64::consts::PI;

// ----------------- 各个形状 -----------------
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// ----------------- Shape 枚举 -----------------
#[derive(enum_visitor::VisitEnum)]
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        // 由 #[derive(VisitEnum)] 在当前模块注入的局部宏：
        visit!(self, |s| s.area())
    }
}

fn main() {
    let c = Shape::Circle(Circle { radius: 2.0 });
    let r = Shape::Rectangle(Rectangle {
        width: 3.0,
        height: 4.0,
    });
    let t = Shape::Triangle(Triangle {
        base: 10.0,
        height: 2.0,
    });

    assert!((c.area() - (PI * 4.0)).abs() < 1e-9);
    assert_eq!(r.area(), 12.0);
    assert_eq!(t.area(), 10.0);
}
