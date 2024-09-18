use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass]
pub struct WebAppUser {
    #[pyo3(get)]
    id: i64,
    #[pyo3(get)]
    is_bot: Option<bool>,
    #[pyo3(get)]
    first_name: String,
    #[pyo3(get)]
    last_name: Option<String>,
    #[pyo3(get)]
    username: Option<String>,
    #[pyo3(get)]
    language_code: Option<String>,
    #[pyo3(get)]
    is_premium: Option<bool>,
    #[pyo3(get)]
    added_to_attachment_menu: Option<bool>,
    #[pyo3(get)]
    allows_write_to_pm: Option<bool>,
    #[pyo3(get)]
    photo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass]
pub struct WebAppChat {
    #[pyo3(get)]
    id: i64,
    #[pyo3(get, name = "type")]
    type_: String,
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    username: Option<String>,
    #[pyo3(get)]
    photo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass]
pub struct WebAppInitData {
    #[pyo3(get)]
    pub query_id: Option<String>,
    #[pyo3(get)]
    pub user: Option<WebAppUser>,
    #[pyo3(get)]
    pub receiver: Option<WebAppUser>,
    #[pyo3(get)]
    pub chat: Option<WebAppChat>,
    #[pyo3(get)]
    pub chat_type: Option<String>,
    #[pyo3(get)]
    pub chat_instance: Option<String>,
    #[pyo3(get)]
    pub start_param: Option<String>,
    #[pyo3(get)]
    pub can_send_after: Option<u64>,
    #[pyo3(get)]
    pub auth_date: Option<u64>,
    #[pyo3(get)]
    pub hash: String,

    pub raw_data: Vec<u8>,
}

#[pymethods]
impl WebAppInitData {
    fn __repr__(&self) -> String {
        format!(
            "WebAppInitData {{ query_id: {:?}, user: {:?}, receiver: {:?}, chat: {:?}, chat_type: {:?}, chat_instance: {:?}, start_param: {:?}, can_send_after: {:?}, auth_date: {:?}, hash: {:?} }}",
            self.query_id,
            self.user,
            self.receiver,
            self.chat,
            self.chat_type,
            self.chat_instance,
            self.start_param,
            self.can_send_after,
            self.auth_date,
            self.hash
        )
    }
}

#[pymethods]
impl WebAppUser {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymethods]
impl WebAppChat {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
