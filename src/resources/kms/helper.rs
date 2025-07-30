use base64::Engine;

// Helper function to encode data as base64
pub fn encode_b64(data: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(data.as_bytes())
}

// Helper function to decode base64 data
pub fn decode_b64(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(data)?;
    Ok(String::from_utf8(bytes)?)
}
