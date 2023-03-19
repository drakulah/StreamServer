use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{video_info::get_video_info, store::Store};

fn get_timestamp() -> u64 {
  let now = SystemTime::now();
  let mut time: u64 = 0;
  if let Ok(timestamp) = now.duration_since(UNIX_EPOCH) {
    time = timestamp.as_secs();
  }
  return time;
}

pub async fn get(video_id: &str) -> Option<(String, u64)> {
  let s = Store::new("youtube_url");

  // Return if url is in store
  if let Some(content) = s.get(&video_id) {
    let arr = content.split(" && ").collect::<Vec<&str>>();
    if arr.len() == 3 {
      let ts_str = arr[0];
      let file_size = match arr[1].parse::<u64>() {
        Ok(e) => e,
        Err(_) => 0 
      };
      if let Ok(that_ts) = ts_str.parse::<u64>() {
        if (that_ts - 1800) > get_timestamp() {
          return Some((arr[2].to_string(), file_size));
        }
      }
    }
  }

  // Fetch if url is not in store
  if let Some(info) = get_video_info(&video_id).await {
    let mut audio_formats = info.streaming_data.adaptive_formats
      .iter()
      .filter(|&e| Regex::new(r"^audio/(webm|mp4);").unwrap().is_match(&e.mime_type))
      .collect::<Vec<_>>();

    audio_formats.sort_by(|a, b| b.bitrate.cmp(&a.bitrate));

    if let Some(format) = audio_formats.get(0) {
      let mut file_size: u64 = 0;

      if let Ok(s) = format.content_length.parse::<u64>() {
        file_size = s;
      }

      if let Ok(exp_after) = info.streaming_data.expires_in_seconds.parse::<u64>() {
        s.set(video_id, &format!("{} && {} && {}", exp_after + get_timestamp(), file_size, format.url));
      }

      return Some((format.url.to_string(), file_size));
    }
  }

  None
}