use std::{convert::Infallible, net::SocketAddr};
use hyper::{service, Body, Method, Request, Response, Server};

mod audio_url;
mod constants;
mod query_parser;
mod store;
mod stream;
mod video_info;
mod yt_client;

async fn middleware(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let uri = req.uri().clone();
  let method = req.method().clone();
  let path = uri.path().to_lowercase().to_owned();
  let query = match uri.query() {
      Some(q) => q,
      None => "",
  };
  let r = Response::new(Body::from("404 Not found!"));

  if method == Method::GET {
    match path.as_str() {
      "/listen" => {
        let query_params = query_parser::parse_query(query);
        if let Some(raw_id) = query_params.get("id") {
          let id = raw_id.to_owned();
          if let Some(stream_res) = stream::get_stream_as_response(&id, req).await {
            return Ok(stream_res);
          }
        }
        return Ok(r);
      }
      _ => { // for all "/*" routes
        if let Some(video_id) = uri.path().to_owned().strip_prefix("/stream/") {

          if let Some(stream_res) = stream::get_stream_as_response(video_id, req).await {
            return Ok(stream_res);
          } 
        }
        
      }
    }
  }

  if method == Method::POST {}

  Ok(r)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();

  let service = service::make_service_fn(|_| async {
    Ok::<_, Infallible>(service::service_fn(middleware))
  });

  let server = Server::bind(&addr).serve(service);
  println!("Started server at http://{}", addr);
  server.await?;
  Ok(())
}
