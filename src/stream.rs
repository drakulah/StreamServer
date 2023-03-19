use crate::audio_url;
use async_recursion::async_recursion;
use hyper_rustls::HttpsConnectorBuilder;
use hyper::{Client, Request, Body, Response, HeaderMap, http::HeaderValue};

#[allow(dead_code)]
fn first_chunk_end(size: u64) -> u64 {
  if size == 0 {
    return 0;
  }

  if size > 204_800 {
    return 204_800;
  }

  size - 1
}

#[async_recursion]
async fn fetch(uri: &str, req_headers: &HeaderMap, follow_redirect: bool) -> Option<Response<Body>> {
  let https_connector = HttpsConnectorBuilder::default()
    .with_native_roots()
    .https_or_http()
    .enable_http1()
    .build();

  let mut req = Request::builder().uri(uri);
  let client = Client::builder().build::<_, hyper::Body>(https_connector);

  for (k, v) in req_headers.iter() {
    if let Some(it) = req.headers_mut() {
      it.insert(k, v.to_owned());
    }
  }

  if let Ok(res) = client.request(req.body(Body::empty()).unwrap()).await {
    if follow_redirect && res.status().is_redirection() {
      if let Some(location) = res.headers().get("location") {
        if let Ok(uri) = location.to_str() {
          let re_req = fetch(uri, req_headers, follow_redirect).await;
          return re_req;
        }
      }
    }
    return Some(res);
  }

  None
}

pub async fn get_stream_as_response(video_id: &str, w: Request<Body>) -> Option<Response<Body>> {
  if let Some((audio_uri, file_size)) = audio_url::get(video_id).await {
    
    let client_headers = w.headers().clone();
    let mut header = HeaderMap::new();
    header.insert("accept", HeaderValue::from_static("*/*"));
    header.insert("pragma", HeaderValue::from_static("no-cache"));
    header.insert("connection", HeaderValue::from_static("keep-alive"));
    header.insert("cache-control", HeaderValue::from_static("no-cache"));
    header.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.5"));
    header.insert("accept-encoding", HeaderValue::from_static("gzip, deflate, br"));
    header.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    header.insert("sec-fetch-mode", HeaderValue::from_static("no-cors"));
    header.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    header.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"));

    if let Some(range) = client_headers.get("range") {
      if let Ok(v) = range.to_str() {
        if let Ok(range) = HeaderValue::from_str(v) {
          header.insert("range", range);
        }
      }
    }

    if let Some(dest) = client_headers.get("sec-fetch-dest") {
      if let Ok(v) = dest.to_str() {
        match v {
          "empty" => {
            header.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
          },
          _ => {
            if let Ok(fuck) = HeaderValue::from_str(format!("bytes=0-{}/{}", file_size - 1, file_size).as_str()) {
              header.insert("range", fuck);
            }
            header.insert("sec-fetch-dest", HeaderValue::from_static("video"));
          }
        };
      }
    }

    return fetch(&audio_uri, &header, true).await;
  }

  None
}