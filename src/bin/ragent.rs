extern crate ragent;
extern crate serde_json;
extern crate hyper;

use ragent::get_filesystems;
use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {
    let addr = ([0, 0, 0, 0], 21488).into();
    let new_service = || {
        service_fn_ok(|_| {
            Response::new(Body::from(serde_json::to_string(&get_filesystems()).unwrap()))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
