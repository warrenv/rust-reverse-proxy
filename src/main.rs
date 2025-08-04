use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

type HttpClient = Client<hyper::client::HttpConnector>;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("rev-proxy listening on {:?}", addr);

    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(_client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("req: {:?}", req);
    let headers = req.headers().clone();
    println!("headers: {:?}", headers);

    let path = req.uri().path().to_string();
    println!("proxy request for path: {:?}", path);

    //    if path.starts_with("/hello") {
    //        let target_url = "http://127.0.0.1:4000".to_owned();
    //        let resp = get_response(_client, req, &target_url, &path).await?;
    //        return Ok(resp);
    //    }
    let target_url = "http://127.0.0.1:4000".to_owned();
    let resp = get_response(_client, req, &target_url, &path).await?;
    println!("resp: {:?}", resp);
    return Ok(resp);

    //    let resp = Response::new(Body::from("sorry! no route found"));
    //    Ok(resp)
}

async fn get_response(
    client: HttpClient,
    req: Request<Body>,
    target_url: &str,
    path: &str,
) -> Result<Response<Body>, hyper::Error> {
    let target_url = format!("{}{}", target_url, path);
    let fallback_url = format!("http://127.0.0.1:3000{}", path);
    let headers = req.headers().clone();

    let head_request_builder = Request::builder()
        .method(http::Method::HEAD)
        .uri(target_url.clone())
        .body(Body::empty())
        .unwrap();

    // Try a HEAD request to the wordpress server.
    // If statusCode == 200, make the real request to wordpress.
    // If statusCode == 404, make a request to the frontend.

    let request_builder = match client.request(head_request_builder).await {
        Ok(resp) => {
            println!("HEAD resp: {:?}", resp);
            println!("   status: {:?}", resp.status());
            if resp.status() == 200 {
                Ok(Request::builder()
                    .method(req.method())
                    .uri(target_url)
                    .body(req.into_body())
                    .unwrap())
            //} else if resp.status() == 404 {
            } else {
                println!("wordpress 404, send to frontend");
                Ok(Request::builder()
                    .method(req.method())
                    .uri(fallback_url)
                    .body(req.into_body())
                    .unwrap())
            }
            //            } else {
            //                panic!("head not 200 or 404");
            //            }
        }
        Err(e) => {
            println!("HEAD fail: {:?}", e);
            Err(e)
        }
    };

    match request_builder {
        Ok(mut request_builder) => {
            *request_builder.headers_mut() = headers;
            let response = client.request(request_builder).await?;
            let body = hyper::body::to_bytes(response.into_body()).await?;
            let body = String::from_utf8(body.to_vec()).unwrap();

            let mut resp = Response::new(Body::from(body));
            //let mut resp = Response::new(Body::from("".to_string()));
            *resp.status_mut() = http::StatusCode::OK;
            Ok(resp)
        }
        Err(e) => {
            let mut resp = Response::new(Body::from(e.to_string()));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;
            Ok(resp)
        }
    }
}
