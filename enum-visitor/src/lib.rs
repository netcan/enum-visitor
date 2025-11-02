//! enum-visitor: emulate a `std::visit`-like experience with macros.
//!
//! Two ways to use:
//! - `visit_with!(expr, Enum, [V1, V2, ...], |binder| expr)` universal macro
//! - `#[derive(VisitEnum)]` -> `visit_with_<enum>!(expr, |binder| expr)`

pub use enum_visitor_derive::VisitEnum;

/// 通用版本：需要显式给出枚举类型与变体列表。
/// 支持表达式或块体形式的回调；`binder` 是每个变体内部单元素元组字段的绑定名。
///
/// 约束：当前仅支持“单元素元组变体”（如 `Enum::Variant(T)`）。
#[macro_export]
macro_rules! visit_with {
    // 表达式体形式
    ($expr:expr, $Enum:path, [$($Var:ident),+ $(,)?], |$b:pat_param| $body:expr $(,)?) => {
        match $expr {
            $( $Enum::$Var($b) => { $body } ),+
        }
    };
    // 块体形式
    ($expr:expr, $Enum:path, [$($Var:ident),+ $(,)?], |$b:pat_param| { $($body:tt)* } $(,)?) => {
        match $expr {
            $( $Enum::$Var($b) => { $($body)* } ),+
        }
    };
}
