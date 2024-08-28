use std::{future::Future, net::SocketAddr};

use derive_new::new;
use nakago::{Inject, Tag};
use nakago_figment::FromRef;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

use crate::{errors, Config};

// Server Initialization
// ---------------------

/// TCP Listener Initialization
#[derive(Debug, Clone, Default, new)]
pub struct Listener<C: nakago_figment::Config> {
    config_tag: Option<&'static Tag<C>>,
}

impl<C: nakago_figment::Config> Listener<C> {
    /// Initialize the TCP Listener
    pub async fn init(
        &self,
        i: &Inject,
        filter: BoxedFilter<(impl Reply + 'static,)>,
    ) -> nakago::Result<(impl Future<Output = ()>, SocketAddr)>
    where
        Config: FromRef<C>,
    {
        let config = if let Some(tag) = self.config_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<C>().await?
        };

        let http = Config::from_ref(&*config);

        let addr: SocketAddr = format!("0.0.0.0:{}", http.port)
            .parse()
            .expect("Unable to parse bind address");

        let (actual_addr, server) = warp::serve(
            filter
                .with(warp::log("warp"))
                .recover(errors::handle_rejection),
        )
        .bind_ephemeral(addr);

        Ok((server, actual_addr))
    }
}
