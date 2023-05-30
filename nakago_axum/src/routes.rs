use std::sync::Mutex;

use async_trait::async_trait;
use axum::Router;
use hyper::Body;
use nakago::inject;

use crate::app::State;

/// A function to generate a new Router instance
pub type GetRouter<S, B> = fn(&inject::Inject) -> Router<S, B>;

pub(crate) type Routers<S, B = Body> = Vec<Mutex<Router<S, B>>>;

/// A hook to initialize a particular route
pub struct InitRouter<S: State, B = Body> {
    get_router: GetRouter<S, B>,
}

impl<S: State, B> InitRouter<S, B> {
    /// Create a new InitRouter instance
    pub const fn new(get_router: GetRouter<S, B>) -> Self {
        Self { get_router }
    }
}

#[async_trait]
impl<S: State, B: 'static> inject::Hook for InitRouter<S, B> {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        let router: Router<S, B> = (self.get_router)(i);

        if let Some(routes) = i.get_type_mut_opt::<Routers<S, B>>() {
            routes.push(Mutex::new(router));
        } else {
            i.inject_type(vec![Mutex::new(router)])?;
        }

        Ok(())
    }
}
