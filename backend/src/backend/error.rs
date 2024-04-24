use hyper::body::Bytes;
use hyper::StatusCode;
use thiserror::Error;

use super::http::SocketResponse;

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("Cannot connected to '{0}', because '{1}'")]
    UnixSocketConnect(String, std::io::Error),

    #[error("Cannot perform handshake to '{0}', because '{1}'")]
    HandshakeFailed(String, hyper::Error),

    #[error("Cannot build HTTP request to '{0}', because '{1}'")]
    BuilderFailed(String, hyper::http::Error),

    #[error("Cannot clean HTTP connection to '{0}', because '{1}'")]
    ConnectionFailed(String, hyper::Error),

    #[error("Cannot join HTTP connection to '{0}', because '{1}'")]
    TokioFailed(String, tokio::task::JoinError),

    #[error("Cannot send HTTP request to '{0}', because '{1}'")]
    RequestFailed(String, hyper::Error),

    #[error("Cannot accept HTTP status code from '{0}', because '{1}'")]
    StatusFailed(String, hyper::http::StatusCode, SocketResponse),

    #[error("Cannot receive HTTP response from '{0}', because '{1}'")]
    ResponseFailed(String, hyper::Error),

    #[error("Cannot deserialize JSON payload from '{0:?}', because '{1}'")]
    DeserializationFailed(Option<hyper::http::StatusCode>, serde_json::Error, Bytes),
}

pub type SocketResult<T> = Result<T, SocketError>;

impl SocketError {
    pub(crate) fn raise_unix_socket_connect<T>(
        socket: &str,
        error: std::io::Error,
    ) -> SocketResult<T> {
        Err(Self::UnixSocketConnect(socket.to_owned(), error))
    }

    pub(crate) fn raise_handshake_failed<T>(socket: &str, error: hyper::Error) -> SocketResult<T> {
        Err(Self::HandshakeFailed(socket.to_owned(), error))
    }

    pub(crate) fn raise_builder_failed<T>(url: &str, error: hyper::http::Error) -> SocketResult<T> {
        Err(Self::BuilderFailed(url.to_owned(), error))
    }

    pub(crate) fn raise_connection_failed<T>(url: &str, error: hyper::Error) -> SocketResult<T> {
        Err(Self::ConnectionFailed(url.to_owned(), error))
    }

    pub(crate) fn raise_tokio_failed<T>(
        url: &str,
        error: tokio::task::JoinError,
    ) -> SocketResult<T> {
        Err(Self::TokioFailed(url.to_owned(), error))
    }

    pub(crate) fn raise_request_failed<T>(url: &str, error: hyper::Error) -> SocketResult<T> {
        Err(Self::RequestFailed(url.to_owned(), error))
    }

    pub(crate) fn raise_status_failed<T>(
        status: hyper::http::StatusCode,
        response: SocketResponse,
    ) -> SocketResult<T> {
        Err(Self::StatusFailed(
            response.url.to_owned(),
            status,
            response,
        ))
    }

    pub(crate) fn raise_response_failed<T>(url: &str, error: hyper::Error) -> SocketResult<T> {
        Err(Self::ResponseFailed(url.to_owned(), error))
    }

    pub(crate) fn raise_deserialization_failed<T>(
        status: Option<StatusCode>,
        error: serde_json::Error,
        data: Bytes,
    ) -> SocketResult<T> {
        Err(Self::DeserializationFailed(status, error, data))
    }
}
