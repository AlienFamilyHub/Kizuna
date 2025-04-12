use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

// 定义常量
const HIDDEN_CONTENT: &str = "[BASE64_CONTENT_HIDDEN]";

pub fn report(
    endpoint: &str,
    token: &str,
) -> (
    String,
    HashMap<String, Value>,
    String,
    HashMap<String, String>,
    String,
) {
    // 获取配置
    let config = crate::modules::get_config::load_config();

    // 从 get_processes 模块获取当前前台进程名称和窗口名称以及icon_base64
    let (process_name, window_name, icon_base64) = crate::modules::get_processes::get_window_info();

    // 自定义程序名：从配置文件中读取规则，替换进程名
    let process_name = crate::modules::get_processes::replacer(&process_name.replace(".exe", ""));

    // 获取媒体信息
    let (title, artist, source_app_name, album_title, album_artist, album_thumbnail) =
        crate::modules::get_media::get_media_info();

    // 构建媒体更新请求，根据配置决定是否包含封面信息
    let album_thumbnail_to_use = if config.server_config.skip_smtc_cover {
        // 如果配置为跳过SMTC封面，则不在媒体更新请求中包含封面信息
        String::new()
    } else {
        // 使用album_thumbnail，它可能是base64数据或者是上传后的URL
        album_thumbnail.clone()
    };

    let media_update = crate::modules::requests::build_media_update(
        &title,
        &artist,
        &source_app_name,
        &album_title,
        &album_artist,
        &album_thumbnail_to_use,
    );
    // 将上一步的媒体信息同程序名构建请求数据
    let mut update_data =
        crate::modules::requests::build_data(&process_name, media_update.clone(), token);

    // 发送请求并记录日志
    let response = crate::modules::requests::report(update_data.clone(), endpoint);

    let status = !config.server_config.log_base64
        && config.server_config.report_smtc
        && !config.server_config.skip_smtc_cover
        && !config.server_config.upload_smtc_cover;

    let log_message = if status {
        // 首先尝试JSON解析方式
        if let Ok(mut json) = serde_json::from_str::<Value>(&response) {
            if let Some(media) = json.get_mut("media") {
                if let Some(album_thumbnail) = media.get_mut("AlbumThumbnail") {
                    if let Value::String(thumb_str) = album_thumbnail {
                        if thumb_str.contains("base64") {
                            *album_thumbnail = Value::String(HIDDEN_CONTENT.to_string());
                        }
                    }
                }
            }
            json.to_string()
        } else {
            lazy_static! {
                static ref BASE64_RE: Regex = Regex::new(
                    r#"(?i)("AlbumThumbnail":\s*")([^"]*base64[^"]*)(")|(data:image/[^;]+;base64,[^"]+)"#
                ).expect("Invalid regex pattern");
            }

            BASE64_RE
                .replace_all(&response, |caps: &regex::Captures| {
                    if let Some(_) = caps.get(2) {
                        format!("{}{}{}", &caps[1], HIDDEN_CONTENT, &caps[3])
                    } else {
                        format!("data:image/type;base64,{}", HIDDEN_CONTENT)
                    }
                })
                .to_string()
        }
    } else {
        response.to_string()
    };

    log::info!("{}", log_message);

    // 移除构建数据当中的 key 字段
    update_data.remove("key");
    // 插入窗口名称
    update_data.insert(
        "window_name".to_string(),
        serde_json::Value::String(window_name.trim_end_matches('\u{0000}').to_string()),
    );

    (
        response,
        update_data,
        icon_base64,
        media_update,
        album_thumbnail,
    )
}
