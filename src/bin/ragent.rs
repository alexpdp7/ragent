extern crate hyper;
extern crate ragent;
extern crate serde_json;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use ragent::get_ragent_info;

fn main() {
    let addr = ([0, 0, 0, 0], 21488).into();
    let new_service = || {
        service_fn_ok(|_| {
            Response::new(Body::from(
                serde_json::to_string(&get_ragent_info()).unwrap(),
            ))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
