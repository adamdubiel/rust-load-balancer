
extern crate router;
extern crate iron;

pub use self::router::Router;
pub use std::sync::Arc;

use self::iron::prelude::{Request, Response, IronResult};
use self::iron::status;
use resolver::GroupResolver;

pub fn register(router: &mut Router, resolver: Arc<GroupResolver>) {
    router.get("/groups/:customer", Controller { resolver: resolver.clone() });
}

fn customer_group_handler(req: &mut Request, resolver: &GroupResolver) -> IronResult<Response> {
    let customer = match req.extensions.get::<Router>() {
        Some(p) => { p.find("customer") },
        _ => { println!("Failed to find any group in request, ignoring"); None }
    };

    let group = match customer {
        Some(c) => { resolver.resolve_group(c.to_string()) },
        _ => { println!("Failed to read customer"); "default".to_string() }
    };

    Ok(Response::with((status::Ok, group)))
}

struct Controller {
    resolver: Arc<GroupResolver>
}

impl iron::middleware::Handler for Controller {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let resolver = &*self.resolver;
        customer_group_handler(req, resolver)
    }

}
