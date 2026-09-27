use std::fmt;

use gtk::glib;
use webkit6::soup;
use webkit6::soup::prelude::*;

pub enum DownloadError {
    InvalidAddress(glib::BoolError),
    Transfer(glib::Error),
    Status(soup::Status),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddress(error) => write!(formatter, "endereço inválido: {error}"),
            Self::Transfer(error) => write!(formatter, "falha na transferência: {error}"),
            Self::Status(status) => write!(formatter, "resposta HTTP inesperada: {status:?}"),
        }
    }
}

pub async fn fetch(uri: &str) -> Result<glib::Bytes, DownloadError> {
    let message = soup::Message::new("GET", uri).map_err(DownloadError::InvalidAddress)?;
    let body = soup::Session::new()
        .send_and_read_future(&message, glib::Priority::DEFAULT)
        .await
        .map_err(DownloadError::Transfer)?;
    match message.status() {
        soup::Status::Ok => Ok(body),
        status => Err(DownloadError::Status(status)),
    }
}
