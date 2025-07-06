#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

use hyper_util::rt::{TokioIo, TokioTimer};

use ragent::get_ragent_info;

async fn service(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(
        serde_json::to_string(&get_ragent_info()).unwrap().into(),
    )))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 21488).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{addr}");
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(service))
                .await
            {
                println!("Error serving connection: {err:?}");
            }
        });
    }
}
