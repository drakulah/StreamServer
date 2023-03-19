use serde_json::json;
use crate::constants::constants::VISITOR_DATA;

#[allow(dead_code)]
pub fn android_youtube_music(video_id: &str) -> String {
  json!({ 
    "videoId": video_id,
    "context": {
      "client": {
        "androidSdkVersion": 31,
        "gl"               : "US",
        "hl"               : "en",
        "clientVersion"    : "5.26.1",
        "clientName"       : "ANDROID_MUSIC",
        "visitorData"      : VISITOR_DATA
      }
    }
  }).to_string()
}

#[allow(dead_code)]
pub fn android_youtube(video_id: &str) -> String {
  json!({ 
    "videoId": video_id,
    "context": {
      "client": {
        "androidSdkVersion": 31,
        "gl"               : "US",
        "hl"               : "en",
        "clientVersion"    : "17.36.4",
        "clientName"       : "ANDROID",
        "visitorData"      : VISITOR_DATA
      }
    }
  }).to_string()
}

#[allow(dead_code)]
pub fn web_youtube(video_id: &str) -> String {
  json!({ 
    "videoId": video_id,
    "context": {
      "client": {
        "gl"               : "US",
				"hl"               : "en",
				"clientVersion"    : "2.20220918",
				"clientName"       : "WEB",
        "visitorData"      : VISITOR_DATA
      }
    }
  }).to_string()
}