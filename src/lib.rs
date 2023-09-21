//! just a macro to get qualified function name
//!
//! note: this doesn't work with `const` due to,
//! [#63084](https://github.com/rust-lang/rust/issues/63084)

/// consider vendoring this code in your project, rather than using this library
#[macro_export]
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}
