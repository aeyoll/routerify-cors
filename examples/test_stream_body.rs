use hyper::{Body as HyperBody, Request, Response, Server};
use routerify::{Router, RouterService};
use routerify_cors::enable_cors_all;
use std::{convert::Infallible, net::SocketAddr};
use stream_body::StreamBody;

async fn home_handler(_: Request<HyperBody>) -> Result<Response<StreamBody>, Infallible> {
    Ok(Response::new(StreamBody::from("Home page")))
}

fn router() -> Router<StreamBody, Infallible> {
    Router::builder()
        .middleware(enable_cors_all())
        .get("/", home_handler)
        .options(
            "/*",
            |_req| async move { Ok(Response::new(StreamBody::from("Options"))) },
        )
        .any(|_req| async move { Ok(Response::new(StreamBody::from("Not Found"))) })
        .err_handler(|err| async move { Response::new(StreamBody::from(format!("Error: {}", err))) })
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    let service = RouterService::new(router).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
