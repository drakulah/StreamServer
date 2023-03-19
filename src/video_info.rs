use regex::Regex;
use serde::{Deserialize, Serialize};
use hyper_rustls::HttpsConnectorBuilder;
use crate::{yt_client, constants::constants::KEY_AND_MUSIC};
use hyper::{Client, Request, header::CONTENT_TYPE, Body, body::to_bytes};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Range {
  pub start: String,
  pub end: String
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveFormat {
  pub itag: i32,
  pub url: String,
  pub mime_type: String,
  pub bitrate: usize,
  pub average_bitrate: usize,
  pub content_length: String
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
  pub expires_in_seconds: String,
  pub adaptive_formats: Vec<AdaptiveFormat>
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
  pub streaming_data: StreamingData
}

pub fn is_valid_video_id(video_id: &str) -> bool {
  let id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
  id_regex.is_match(video_id)
}

pub async fn get_video_info(video_id: &str) -> Option<VideoInfo> {

  if !is_valid_video_id(video_id) {
    return None;
  }

  let https_connector = HttpsConnectorBuilder::default()
    .with_native_roots()
    .https_or_http()
    .enable_http1()
    .build();

  let client = Client::builder()
    .build::<_, hyper::Body>(https_connector);

  if let Ok(req) = Request::builder()
    .method("POST")
    .uri("https://music.youtube.com/youtubei/v1/player?prettyPrint=false")
    .header(CONTENT_TYPE, "application/json")
    .header("X-Goog-Api-Key", KEY_AND_MUSIC)
    .body(Body::from(yt_client::android_youtube_music(video_id))
  ) {
    match client.request(req).await {
      Ok(res) => {
        if let Ok(body) = to_bytes(res.into_body()).await {
          let try_parse: Result<VideoInfo, serde_json::Error> = serde_json::from_slice(&body);
          return match try_parse {
            Ok(res_json) => Some(res_json),
            Err(_) => None
          };
        }
      },
      Err(e) => {
        println!("{:?}", e);
      }
    }
  }

  None
}
