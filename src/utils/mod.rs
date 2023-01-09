use base64::Engine;

pub mod options;


pub fn basic_header(username: &str, password: &str)->String{
    base64::engine::general_purpose::STANDARD.encode(
        format!(
            "{}:{}",
            username,
            password
        )
    )
}