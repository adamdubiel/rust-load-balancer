
extern crate iron;
extern crate router;

use self::router::Router;
use self::iron::Iron;

pub fn start_server(router: Router) {
    Iron::new(router).http("localhost:3000")
        .ok()
        .expect("Something went terribly wrong :(");
}
