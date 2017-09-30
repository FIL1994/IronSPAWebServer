#![allow(non_snake_case)]
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate router;

use iron::{Iron, Request, Response, IronResult, Handler, status, AroundMiddleware, Chain};
use iron::mime::Mime;
use iron::modifier::Modifier;

use mount::Mount;
use staticfile::{Static, Cache};
use std::path::Path;
use std::fs::File;
use std::time::Duration;

struct Fallback;

/// Middleware for handling 404 errors
impl AroundMiddleware for Fallback {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(FallbackHandler(handler))
    }
}

struct FallbackHandler(Box<Handler>);

/// Handles 404 errors
/// Serves the index.html page which will handle routing
impl Handler for FallbackHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let resp = self.0.handle(req);
        match resp {
            Err(err) => {
                match err.response.status {
                    Some(status::NotFound) => {
                        let content_type = "text/html".parse::<Mime>().unwrap();
                        let file = File::open("../html/index.html").unwrap();
                        Ok(Response::with((content_type, status::Ok, file)))
                    }
                    _ => Err(err),
                }
            }
            other => other
        }
    }
}

/// Main function
/// Starts the server
fn main() {
    let mut mount = Mount::new();
    let cache_time = Duration::new(7*24*2600, 0); //one week
    let mut staticFiles = Static::new(Path::new("../html"));
    Cache::new(cache_time).modify(&mut staticFiles);

    mount
        .mount("/", staticFiles);
        //.mount("*", router);

    let mut chain = Chain::new(mount);
    chain.link_around(Fallback);

    println!("Starting Iron Server on port 1393");
    Iron::new(chain).http("localhost:1393").unwrap();
}