#![feature(async_await, await_macro, futures_api)]

use {
    hyper::{
        Body, Client, Request, Response, Server, Uri,
        service::service_fn,
        rt::run,
    },
    futures::{
        compat::TokioDefaultSpawner,
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
    tokio::await,
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path_and_query = match req.uri().path_and_query() {
        Some(path_and_query) => path_and_query,
        None => return Ok(Response::new(Body::from("invalid URL query"))),
    };

    let url = format!("http://google.com{}", path_and_query.as_str());

    let url = match url.parse::<Uri>() {
        Ok(url) => url,
        Err(e) => return Ok(Response::new(Body::from(
            format!("failed to parse URL: {:?}", e)))),
    };
    // Return the result of the request directly to the user
    await!(Client::new().get(url))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req|
            serve_req(req).boxed().compat(TokioDefaultSpawner)
        ));

    if let Err(e) = await!(serve_future) {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let futures_03_future = run_server(addr);
    let futures_01_future =
        futures_03_future.unit_error().boxed().compat(TokioDefaultSpawner);
    run(futures_01_future);
}
