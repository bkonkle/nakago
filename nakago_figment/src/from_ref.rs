// Taken from: https://github.com/tokio-rs/axum/blob/main/axum-core/src/extract/from_ref.rs

/// Used to do reference-to-value conversions thus not consuming the input value.
///
/// This trait can be derived using `#[derive(FromRef)]`.
pub trait FromRef<T> {
    /// Converts to this type from a reference to the input type.
    fn from_ref(input: &T) -> Self;
}

impl<T> FromRef<T> for T
where
    T: Clone,
{
    fn from_ref(input: &T) -> Self {
        input.clone()
    }
}
