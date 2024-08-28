use nakago::Inject;
use warp::{filters::BoxedFilter, Filter};

/// Inject the Nakago service into the HTTP route
pub fn with_injection(i: Inject) -> BoxedFilter<(Inject,)> {
    warp::any().map(move || i.clone()).boxed()
}
