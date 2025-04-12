use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;

pub fn upload_smtc_cover(_image_data: &[u8]) -> String {
    // TODO: 实现上传封面功能
    "".to_string()
}

// 将图像转换为WebP格式
fn _convert_to_webp(image_data: &[u8]) -> Result<Vec<u8>, String> {
    let img = match ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|e| format!("无法识别图像格式: {}", e))?
        .decode()
        .map_err(|e| format!("解码图像失败: {}", e))
    {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    let mut webp_data = Vec::new();
    let mut cursor = Cursor::new(&mut webp_data);

    img.write_to(&mut cursor, ImageFormat::WebP)
        .map_err(|e| format!("转换为WebP失败: {}", e))?;

    Ok(webp_data)
}
