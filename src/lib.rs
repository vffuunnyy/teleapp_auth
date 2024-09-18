mod webapp_data;

use hmac::{Hmac, Mac};
use pyo3::prelude::*;
use sha2::Sha256;
use std::collections::HashMap;
use webapp_data::{WebAppChat, WebAppInitData, WebAppUser};

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[pyfunction]
fn validate_webapp_data(webapp_data: WebAppInitData, secret_key: Vec<u8>) -> PyResult<bool> {
    let hash = hmac_sha256(&secret_key, &webapp_data.raw_data);
    let hash_str = hex::encode(hash);

    Ok(hash_str == webapp_data.hash)
}

#[pyfunction]
fn get_secret_key(bot_token: String) -> Vec<u8> {
    hmac_sha256(b"WebAppData", bot_token.as_bytes())
}

#[pyfunction]
fn parse_webapp_data(webapp_data: String) -> WebAppInitData {
    let raw: HashMap<String, String> =
        serde_urlencoded::from_str(&webapp_data).expect("Unable to parse URL");

    let res: WebAppInitData = WebAppInitData {
        query_id: raw.get("query_id").map(|x| x.to_string()),
        user: raw
            .get("user")
            .map(|x| serde_json::from_str(x).expect("Unable to parse user")),
        receiver: raw
            .get("receiver")
            .map(|x| serde_json::from_str(x).expect("Unable to parse receiver")),
        chat: raw
            .get("chat")
            .map(|x| serde_json::from_str(x).expect("Unable to parse chat")),
        chat_type: raw.get("chat_type").map(|x| x.to_string()),
        chat_instance: raw.get("chat_instance").map(|x| x.to_string()),
        auth_date: raw
            .get("auth_date")
            .map(|x| x.parse::<u64>().expect("Unable to parse auth_date")),
        hash: raw
            .get("hash")
            .map(|x| x.to_string())
            .expect("Hash is required"),
        start_param: raw.get("start_param").map(|x| x.to_string()),
        can_send_after: raw
            .get("can_send_after")
            .map(|x| x.parse::<u64>().expect("Unable to parse can_send_after")),
        raw_data: {
            let mut raw_data: Vec<String> = raw
                .iter()
                .filter_map(|(key, value)| {
                    if key != "hash" && !value.is_empty() {
                        Some(format!("{}={}", key, value))
                    } else {
                        None
                    }
                })
                .collect();
            raw_data.sort_unstable();
            raw_data.join("\n").as_bytes().to_vec()
        },
    };

    res
}

#[pymodule]
fn teleapp_auth(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WebAppUser>()?;
    m.add_class::<WebAppChat>()?;
    m.add_class::<WebAppInitData>()?;

    m.add_function(wrap_pyfunction!(validate_webapp_data, m)?)?;
    m.add_function(wrap_pyfunction!(get_secret_key, m)?)?;
    m.add_function(wrap_pyfunction!(parse_webapp_data, m)?)?;
    Ok(())
}
