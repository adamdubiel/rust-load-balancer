
mod api;
mod infra;
mod resolver;

use infra::{server, config};
use api::handlers;
use api::handlers::Router;
use resolver::GroupResolver;
use std::sync::Arc;

fn main() {
    let config = config::read_configuration("config.toml".to_string());
    let resolver = Arc::new(GroupResolver::new(config));
    let mut router = Router::new();

    handlers::register(&mut router, resolver);
    server::start_server(router);
}
