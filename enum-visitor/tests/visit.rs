use std::f64::consts::PI;

mod shapes_test {
    use super::*;

    #[derive(enum_visitor::VisitEnum)]
    enum Shape {
        Circle(Circle),
        Rectangle(Rectangle),
    }

    struct Circle {
        pub radius: f64,
    }
    impl Circle {
        fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }

    struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    impl Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    impl Shape {
        fn area(&self) -> f64 {
            visit!(self, |s| s.area())
        }
    }

    #[test]
    fn visit_expr_form() {
        let c = Shape::Circle(Circle { radius: 2.0 });
        let r = Shape::Rectangle(Rectangle {
            width: 3.0,
            height: 4.0,
        });
        assert!((c.area() - (PI * 4.0)).abs() < 1e-9);
        assert_eq!(r.area(), 12.0);
    }
}

mod v_mut_test {
    #[derive(enum_visitor::VisitEnum)]
    enum V {
        A(i32),
        B(i32),
    }

    impl V {
        fn inc(&mut self) {
            visit!(self, |ref mut x| {
                *x += 1;
            });
        }
        fn get(&self) -> i32 {
            visit!(self, |x| *x)
        }
    }

    #[test]
    fn visit_block_form_and_mut_binder() {
        let mut v = V::A(1);
        v.inc();
        assert_eq!(v.get(), 2);

        let mut w = V::B(10);
        w.inc();
        assert_eq!(w.get(), 11);
    }
}
