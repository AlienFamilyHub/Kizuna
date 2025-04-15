use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn build_media_update(
    title: &str,
    artist: &str,
    source_app_name: &str,
    thumbnail: &str,
    duration: i64,
    elapsed_time: i64,
) -> HashMap<String, Value> {
    let mut media_update = HashMap::new();
    media_update.insert("title".to_string(), Value::String(title.to_string()));
    media_update.insert("artist".to_string(), Value::String(artist.to_string()));
    media_update.insert(
        "processName".to_string(),
        Value::String(source_app_name.to_string()),
    );
    media_update.insert(
        "AlbumThumbnail".to_string(),
        Value::String(thumbnail.to_string()),
    );
    media_update.insert("duration".to_string(), Value::Number(duration.into()));
    media_update.insert(
        "elapsedTime".to_string(),
        Value::Number(elapsed_time.into()),
    );
    media_update
}

pub fn build_data(
    process_name: &str,
    media_update: HashMap<String, Value>,
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
        if title.is_string() && !title.as_str().unwrap_or("").is_empty() {
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
        Ok(resp) => {
            let status = resp.status();

            match resp.text() {
                Ok(text) => {
                    if text.trim().is_empty() {
                        return format!("API 响应：空响应 (状态码: {})", status);
                    }

                    match serde_json::from_str::<HashMap<String, Value>>(&text) {
                        Ok(mut json) => {
                            if let Some(media_value) = media {
                                json.insert("media".to_string(), media_value.clone());
                            }
                            return format!("API 响应：{:?}", json);
                        }
                        Err(e) => {
                            return format!("解析响应时出错：{}，原始响应：{}", e, text);
                        }
                    }
                }
                Err(e) => return format!("读取响应内容时出错：{}", e),
            }
        }
        Err(e) => return format!("发送请求时出错：{}，详细信息：{:?}", e, e),
    }
}
