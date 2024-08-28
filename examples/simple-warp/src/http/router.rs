use nakago::Inject;
use nakago_warp::{auth::subject::with_auth, utils::with_injection};
use warp::{filters::BoxedFilter, reply::Reply, Filter};

use super::{health, user};

/// Initialize the HTTP router
pub fn init(i: &Inject) -> BoxedFilter<(Box<dyn Reply>,)> {
    warp::path("health")
        .and(warp::get())
        .and(with_injection(i.clone()))
        .and_then(health::health_handler)
        .map(|a| Box::new(a) as Box<dyn Reply>)
        .or(warp::path("username")
            .and(warp::get())
            .and(with_auth(with_injection(i.clone())))
            .and_then(user::handle_get_username)
            .map(|a| Box::new(a) as Box<dyn Reply>))
        .unify()
        .boxed()
}
