use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn build_media_update(
    title: &str,
    artist: &str,
    source_app_name: &str,
    album_title: &str,
    album_artist: &str,
    album_thumbnail: &str,
) -> HashMap<String, String> {
    let mut media_update = HashMap::new();
    media_update.insert("title".to_string(), title.to_string());
    media_update.insert("artist".to_string(), artist.to_string());
    media_update.insert("SourceAppName".to_string(), source_app_name.to_string());
    media_update.insert("AlbumTitle".to_string(), album_title.to_string());
    media_update.insert("AlbumArtist".to_string(), album_artist.to_string());
    media_update.insert("AlbumThumbnail".to_string(), album_thumbnail.to_string());
    media_update
}

pub fn build_data(
    process_name: &str,
    media_update: HashMap<String, String>,
    token: &str,
) -> HashMap<String, Value> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs() as i64;

    let mut update_data = HashMap::new();
    update_data.insert("timestamp".to_string(), Value::from(timestamp));
    update_data.insert(
        "process".to_string(),
        Value::from(process_name.trim_end_matches('\0')),
    );
    update_data.insert("key".to_string(), Value::from(token));

    if let Some(title) = media_update.get("title") {
        if !title.is_empty() {
            update_data.insert("media".to_string(), to_value(media_update).unwrap());
        }
    }

    update_data
}

pub fn report(update_data: HashMap<String, Value>, endpoint: &str) -> String {
    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .danger_accept_invalid_certs(true) // 忽略证书验证
        .build()
        .expect("Failed to build client");

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );

    headers.insert(
        HeaderName::from_static("user-agent"),
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; TokaiTeio) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.1 Safari/537.36 Edg/114.0.1823.82 iykrzu/114.514"),
    );

    // 从update_data中提取媒体字段（如果存在）
    let media = update_data.get("media");

    let response = client
        .post(endpoint)
        .headers(headers)
        .json(&update_data)
        .send();

    match response {
        Ok(resp) => match resp.json::<HashMap<String, Value>>() {
            Ok(mut json) => {
                // 打印媒体字段（如果存在）
                if let Some(media_value) = media {
                    json.insert("media".to_string(), media_value.clone());
                }
                // 打印最终的JSON响应
                return format!("API 响应：{:?}", json);
            }
            Err(e) => return format!("解析响应时出错：{}", e),
        },
        Err(e) => return format!("发送请求时出错：{}，详细信息：{:?}", e, e),
    }
}
