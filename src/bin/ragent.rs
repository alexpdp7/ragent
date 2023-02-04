use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
use ragent::get_ragent_info;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 21488).into();
    let new_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(|_| async {
            Ok::<_, Infallible>(Response::new(Body::from(
                serde_json::to_string(&get_ragent_info()).unwrap(),
            )))
        }))
    });
    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{addr}");

    server.await?;

    Ok(())
}
