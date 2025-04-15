#[cfg(target_os = "windows")]
pub mod windows {
    use base64::Engine;
    use std::fs::File;
    use std::io::Write;
    use tokio::runtime::Runtime;
    use windows::core::Result;
    use windows::core::HSTRING;
    use windows::Media::Control::{
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackInfo,
        GlobalSystemMediaTransportControlsSessionTimelineProperties,
    };
    use windows::Storage::Streams::DataReader;

    pub fn get_media_info() -> (String, String, String, String, i64, i64) {
        // 获取配置，检查是否需要上报SMTC信息
        let config = crate::modules::get_config::load_config();
        if !config.server_config.report_smtc {
            return (
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                0,
                0,
            );
        }

        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        match rt.block_on(async_main()) {
            Ok(info) => info,
            Err(e) => {
                log::error!("Failed to get SMTC info: {}", e);
                (
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    0,
                    0,
                )
            }
        }
    }

    async fn async_main() -> Result<(String, String, String, String, i64, i64)> {
        let session_manager =
            GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
        let current_session = session_manager.GetCurrentSession()?;

        // 获取播放信息
        let playback_info: GlobalSystemMediaTransportControlsSessionPlaybackInfo =
            current_session.GetPlaybackInfo()?;
        let playback_status = playback_info.PlaybackStatus()?;
        // 检查播放状态，如果是暂停或其他非播放状态，返回空值
        if playback_status
            != windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
        {
            return Ok((
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                0,
                0,
            ));
        }

        // 获取时间线信息（包含歌曲长度和当前播放位置）
        let timeline_properties: GlobalSystemMediaTransportControlsSessionTimelineProperties =
            current_session.GetTimelineProperties()?;
        let duration_ticks = timeline_properties.EndTime()?.Duration;
        let elapsed_time_ticks = timeline_properties.Position()?.Duration;

        const TICKS_PER_SECOND: i64 = 10_000_000;

        let duration = (duration_ticks as f64 / TICKS_PER_SECOND as f64).round() as i64;
        let elapsed_time = (elapsed_time_ticks as f64 / TICKS_PER_SECOND as f64).round() as i64;

        // 获取媒体属性
        let media_properties_operation = current_session.TryGetMediaPropertiesAsync()?;
        let media_properties = media_properties_operation.get()?;

        let source_app_name_hstring: HSTRING = current_session.SourceAppUserModelId()?.into();
        let title_hstring: HSTRING = media_properties.Title()?.into();
        let artist_hstring: HSTRING = media_properties.Artist()?.into();

        let mut album_thumbnail = "".to_string();

        let thumbnail_ref = media_properties.Thumbnail()?;
        let thumbnail_stream = thumbnail_ref.OpenReadAsync()?.get()?;
        // 使用 DataReader 读取流中的数据
        let data_reader = DataReader::CreateDataReader(&thumbnail_stream)?;
        let stream_size = thumbnail_stream.Size()?;
        // 将 u64 转换为 u32，处理可能的溢出
        let stream_size_u32: u32 = stream_size.try_into().unwrap_or(0);
        if stream_size_u32 > 0 {
            data_reader.LoadAsync(stream_size_u32)?.get()?;
            // 读取字节数据
            let mut thumbnail_data = vec![0u8; stream_size_u32 as usize];
            data_reader.ReadBytes(&mut thumbnail_data)?;

            // 保存本地缓存
            let cache_path = crate::libs::cache::get_cache_directory();
            let cache_file_path = cache_path.join("album_thumbnail.png");

            // 写入文件
            let mut file = File::create(&cache_file_path)?;
            file.write_all(&thumbnail_data)?;

            // 获取配置
            let config = crate::modules::get_config::load_config();

            // 检查是否需要上传封面到S3桶
            if config.server_config.upload_smtc_cover && config.server_config.s3_config.s3_enable {
                // 上传封面并获取URL
                let cover_url =
                    crate::modules::upload_cover::upload_smtc_cover(&thumbnail_data).await;

                // 如果上传成功，使用URL，否则使用base64
                match cover_url {
                    Ok(url) => {
                        album_thumbnail = url;

                        if album_thumbnail.is_empty() {
                            // 数据为空，使用base64
                            album_thumbnail =
                                base64::engine::general_purpose::STANDARD.encode(&thumbnail_data);
                            log::warn!("S3上传返回为空，使用base64");
                        }
                    }
                    Err(_) => {
                        // 上传失败，使用base64
                        album_thumbnail =
                            base64::engine::general_purpose::STANDARD.encode(&thumbnail_data);
                        log::warn!("S3上传失败，使用base64");
                    }
                }
            } else {
                // 不上传，使用base64
                let base64_data = base64::engine::general_purpose::STANDARD.encode(&thumbnail_data);
                // 检测图片类型并添加适当的MIME类型前缀
                let mime_type =
                    if thumbnail_data.len() >= 3 && thumbnail_data[0..3] == [0xFF, 0xD8, 0xFF] {
                        "image/jpeg"
                    } else {
                        "image/png"
                    };
                album_thumbnail = format!("data:{};base64,{}", mime_type, base64_data);
            }
        }

        Ok((
            title_hstring.to_string_lossy().to_owned(),
            artist_hstring.to_string_lossy().to_owned(),
            source_app_name_hstring.to_string_lossy().to_owned(),
            album_thumbnail,
            duration,
            elapsed_time,
        ))
    }
}
