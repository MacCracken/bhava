//! Internal macros for bhava.

/// Generate a `Display` impl for an enum where each variant maps to a static string.
///
/// # Examples
///
/// ```ignore
/// impl_display!(MyEnum {
///     VariantA => "variant a",
///     VariantB => "variant_b",
/// });
/// ```
macro_rules! impl_display {
    ($ty:ty { $($variant:ident => $s:literal),+ $(,)? }) => {
        impl ::core::fmt::Display for $ty {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let s = match self {
                    $( Self::$variant => $s, )+
                    // non_exhaustive enums: future variants fall through
                    #[allow(unreachable_patterns)]
                    _ => return write!(f, "{:?}", self),
                };
                f.write_str(s)
            }
        }
    };
}
