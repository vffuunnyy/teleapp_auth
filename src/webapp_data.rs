use pyo3::{prelude::*, types::PyDict};
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

    pub fn to_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("query_id", self.query_id.as_ref().map(|x| x.to_object(py)))?;
        dict.set_item(
            "user",
            self.user
                .as_ref()
                .map(|x| x.to_dict(py).expect("Unable to convert user to dict")),
        )?;
        dict.set_item(
            "receiver",
            self.receiver
                .as_ref()
                .map(|x| x.to_dict(py).expect("Unable to convert receiver to dict")),
        )?;
        dict.set_item(
            "chat",
            self.chat
                .as_ref()
                .map(|x| x.to_dict(py).expect("Unable to convert chat to dict")),
        )?;
        dict.set_item(
            "chat_type",
            self.chat_type.as_ref().map(|x| x.to_object(py)),
        )?;
        dict.set_item(
            "chat_instance",
            self.chat_instance.as_ref().map(|x| x.to_object(py)),
        )?;
        dict.set_item(
            "start_param",
            self.start_param.as_ref().map(|x| x.to_object(py)),
        )?;
        dict.set_item(
            "can_send_after",
            self.can_send_after.as_ref().map(|x| x.to_object(py)),
        )?;
        dict.set_item(
            "auth_date",
            self.auth_date.as_ref().map(|x| x.to_object(py)),
        )?;
        dict.set_item("hash", self.hash.to_object(py))?;
        Ok(dict.into())
    }
}

#[pymethods]
impl WebAppUser {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn to_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("id", self.id.to_object(py))?;
        dict.set_item("is_bot", self.is_bot.to_object(py))?;
        dict.set_item("first_name", self.first_name.to_object(py))?;
        dict.set_item("last_name", self.last_name.to_object(py))?;
        dict.set_item("username", self.username.to_object(py))?;
        dict.set_item("language_code", self.language_code.to_object(py))?;
        dict.set_item("is_premium", self.is_premium.to_object(py))?;
        dict.set_item(
            "added_to_attachment_menu",
            self.added_to_attachment_menu.to_object(py),
        )?;
        dict.set_item("allows_write_to_pm", self.allows_write_to_pm.to_object(py))?;
        dict.set_item("photo_url", self.photo_url.to_object(py))?;
        Ok(dict.into())
    }
}

#[pymethods]
impl WebAppChat {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn to_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("id", self.id.to_object(py))?;
        dict.set_item("type", self.type_.to_object(py))?;
        dict.set_item("title", self.title.to_object(py))?;
        dict.set_item("username", self.username.to_object(py))?;
        dict.set_item("photo_url", self.photo_url.to_object(py))?;
        Ok(dict.into())
    }
}
