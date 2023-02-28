use async_graphql::{Error, ErrorExtensions, MaybeUndefined};
use fake::{Dummy, Fake, Faker};
use hyper::StatusCode;
use rand::Rng;

/// A convenience function to create a GraphQL error with predictable extension props
pub fn graphql_error(message: &'static str, code: StatusCode) -> Error {
    anyhow!(message).extend_with(|_err, e| {
        e.set("code", code.as_u16());
        e.set("message", code.to_string());
    })
}

/// A convenience function to create a GraphQL error from an existing error, intended to be
/// used with `.map_err()`
pub fn as_graphql_error(
    message: &'static str,
    code: StatusCode,
) -> Box<dyn Fn(anyhow::Error) -> Error> {
    Box::new(move |err| {
        anyhow!(message).extend_with(|_err, e| {
            e.set("code", code.as_u16());
            e.set("message", code.to_string());
            e.set("reason", err.to_string());
        })
    })
}

/// Randomly generate the `MaybeUndefined` type from the async-graphql library
pub fn dummy_maybe_undef<T, R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> MaybeUndefined<T>
where
    T: Dummy<Faker>,
{
    match (0..2).fake_with_rng(rng) {
        0 => MaybeUndefined::Undefined,
        1 => MaybeUndefined::Null,
        _ => MaybeUndefined::Value(T::dummy_with_rng(config, rng)),
    }
}
