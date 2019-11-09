use hyper::{Body, Method, Request, Response, Server, StatusCode, service::service_fn};
use futures::{future, future::FutureResult };
use hyper::rt::Future;

use log::{info, warn, error};

use rendertron_cache_server::*;

fn main() {
    init_log().unwrap();
    init_vars();
    let addr = *RENDERTRON_CACHE_SOCKET;

    hyper::rt::run(future::lazy(move || {
        let cache_server = CacheServer::new();

        let server = Server::bind(&addr)
            .serve(move || {
                let cache_server = cache_server.clone();
                service_fn(move |req| {
                    service(&cache_server, req)
                })
            })
            .map_err(|e| eprintln!("server error: {}", e));

        server
    }))
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = FutureResult<Response<Body>, GenericError>;

fn service(server: &CacheServer, req: Request<Body>) -> ResponseFuture {
    let res = match req.method() {
        &Method::GET => server.retrieve(&req),
        &Method::PUT => server.refresh(&req),
        &Method::DELETE => server.purge(&req),
        _ => server.retrieve(&req)
    };

    let ret = match res {
        Ok(r) => r,
        Err(e) => {
            error!("{} {} -> {}", req.method().as_str(), req.uri(), e);

            match e {
                Error::RequestError(h, e) => {
                    //error!("{} {} -> {}", req.method().as_str(), req.uri(), h, e.to_string());
                },
                _ => {
                    //error!("{} {} -> {}", req.method().as_str(), req.uri(), e);
                }
            }

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    };

    future::ok(ret)
}

