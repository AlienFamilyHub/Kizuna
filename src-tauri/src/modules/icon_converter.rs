pub fn convert_png_to_base64(filename: &str) -> Option<String> {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filename).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    Some(STANDARD.encode(&buffer))
}
