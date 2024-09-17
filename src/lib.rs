use hmac::{Hmac, Mac};
use pyo3::prelude::*;
use sha2::Sha256;
use std::collections::HashMap;

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[pyfunction]
fn validate_webapp_data(
    webapp_data: HashMap<String, String>,
    secret_key: Vec<u8>,
) -> PyResult<bool> {
    let provided_hash = webapp_data
        .get("hash")
        .expect("No hash provided")
        .to_string();

    let mut check_params: Vec<String> = webapp_data
        .iter()
        .filter_map(|(key, value)| {
            if key != "hash" && !value.is_empty() {
                Some(format!("{}={}", key, value))
            } else {
                None
            }
        })
        .collect();

    check_params.sort_unstable();

    let check_string = check_params.join("\n");
    let hash = hmac_sha256(&secret_key, check_string.as_bytes());
    let hash_str = hex::encode(hash);

    Ok(hash_str == provided_hash)
}

#[pyfunction]
fn get_secret_key(bot_token: String) -> Vec<u8> {
    hmac_sha256(b"WebAppData", bot_token.as_bytes())
}

#[pyfunction]
fn parse_webapp_data(webapp_data: String) -> HashMap<String, String> {
    let query_pairs: std::collections::HashMap<String, String> =
        serde_urlencoded::from_str(&webapp_data).expect("Unable to parse URL");

    query_pairs
}

#[pymodule]
fn teleapp_auth(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_webapp_data, m)?)?;
    m.add_function(wrap_pyfunction!(get_secret_key, m)?)?;
    m.add_function(wrap_pyfunction!(parse_webapp_data, m)?)?;
    Ok(())
}
