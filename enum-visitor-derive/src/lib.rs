use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro to generate helper macros for a given enum.
///
/// It generates two local macros in the same module as the enum:
/// - `visit_<enum_snake>!(expr, |v| body)` — unique name, avoids conflicts;
/// - `visit!(expr, |v| body)` — convenient short name for use in the
///   same module/impl; note: if you derive this on multiple enums in the same
///   module，该宏名会发生冲突，请将枚举分到不同 `mod` 中或使用带枚举名的宏。
///
/// 限制：当前仅支持“单元素元组变体”（如 `Enum::Variant(T)`）。
#[proc_macro_derive(VisitEnum)]
pub fn derive_visit_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_ident = input.ident;
    let enum_name = enum_ident.to_string();
    let macro_ident = format_ident!("visit_{}", to_snake_case(&enum_name));

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                enum_ident,
                "VisitEnum can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // Collect variant idents and ensure single unnamed field
    let mut arms_expr = Vec::new();
    let mut arms_block = Vec::new();

    for variant in data_enum.variants.iter() {
        let v_ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                // ok
            }
            _ => {
                return syn::Error::new_spanned(
                    &variant.ident,
                    "VisitEnum only supports tuple variants with exactly 1 field",
                )
                .to_compile_error()
                .into();
            }
        }

        // Expression-body arm: use `$v` and `$body` from the macro invocation site.
        arms_expr.push(quote! { #enum_ident::#v_ident($v) => { $body } });
        // Block-body arm: use `$v` and `$($tt)*` from the invocation site.
        arms_block.push(quote! { #enum_ident::#v_ident($v) => { $($tt)* } });
    }

    // Build the macro definition. We create a local macro (not exported) so its
    // name stays within the user's crate namespace. The name encodes the enum
    // name to avoid collisions.
    let gen = quote! {
        // A helper macro to visit all variants of this enum.
        // Usage: `visit_<enum_snake>!(expr, |v| <use v> )`
        #[allow(non_snake_case, unused_macros)]
        macro_rules! #macro_ident {
            ($expr:expr, |$v:pat_param| $body:expr $(,)?) => {{
                match $expr {
                    #( #arms_expr ),*
                }
            }};
            ($expr:expr, |$v:pat_param| { $($tt:tt)* } $(,)?) => {{
                match $expr {
                    #( #arms_block ),*
                }
            }};
        }

        // Convenience short name within the same module.
        #[allow(unused_macros)]
        macro_rules! visit {
            ($expr:expr, |$v:pat_param| $body:expr $(,)?) => { #macro_ident!($expr, |$v| $body) };
            ($expr:expr, |$v:pat_param| { $($tt:tt)* } $(,)?) => { #macro_ident!($expr, |$v| { $($tt)* }) };
        }
    };

    gen.into()
}

fn to_snake_case(name: &str) -> String {
    // minimal snake_case converter: FooBar -> foo_bar
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 { out.push('_'); }
            for c in ch.to_lowercase() { out.push(c); }
        } else {
            out.push(ch);
        }
    }
    out
}
