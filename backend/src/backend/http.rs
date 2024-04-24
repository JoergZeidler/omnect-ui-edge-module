use std::path::Path;

use hyper::body::{Bytes, Incoming};
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper::{Request, Response, StatusCode};

use http_body_util::{BodyExt, Full};
use hyper_util::rt::TokioIo;
use serde_json::{from_slice, Value};

use tokio::net::UnixStream;
use tokio::spawn;
use tokio::task::JoinHandle;

use super::error::{SocketError, SocketResult};

#[derive(Debug)]
pub(crate) struct SocketResponse {
    pub(crate) url: String,
    pub(crate) inner: Response<Incoming>,
    pub(crate) connection: JoinHandle<Result<(), hyper::Error>>,
}

impl SocketResponse {
    fn new(
        url: &str,
        response: Response<Incoming>,
        connection: JoinHandle<Result<(), hyper::Error>>,
    ) -> Self {
        Self {
            url: url.to_owned(),
            inner: response,
            connection: connection,
        }
    }

    pub async fn into_bytes(self) -> SocketResult<Bytes> {
        let data: Bytes = match self.inner.collect().await {
            Err(error) => return SocketError::raise_response_failed(&self.url, error),
            Ok(value) => value.to_bytes(),
        };

        match self.connection.await {
            Err(error) => return SocketError::raise_tokio_failed(&self.url, error),
            Ok(Err(error)) => return SocketError::raise_connection_failed(&self.url, error),
            _ => (),
        }

        Ok(data)
    }

    pub async fn into_json<T>(self) -> SocketResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status: StatusCode = self.inner.status();
        let data: Bytes = self.into_bytes().await?;
        println!("{:?}", data);

        match from_slice(data.as_ref()) {
            Err(error) => SocketError::raise_deserialization_failed(Some(status), error, data),
            Ok(value) => Ok(value),
        }
    }
}

pub struct SocketConnection {
    sender: SendRequest<Full<Bytes>>,
    connection: JoinHandle<Result<(), hyper::Error>>,
}

impl SocketConnection {
    pub async fn open(socket: &str) -> SocketResult<Self> {
        let stream: TokioIo<UnixStream> = match UnixStream::connect(Path::new(socket)).await {
            Err(error) => return SocketError::raise_unix_socket_connect(socket, error),
            Ok(stream) => TokioIo::new(stream),
        };

        let connection: SocketConnection = match handshake(stream).await {
            Err(error) => return SocketError::raise_handshake_failed(socket, error),
            Ok((sender, connection)) => Self {
                sender: sender,
                connection: spawn(async move { connection.await }),
            },
        };

        Ok(connection)
    }

    async fn execute(
        mut self,
        url: &str,
        request: Request<Full<Bytes>>,
    ) -> SocketResult<SocketResponse> {
        let response: Response<Incoming> = match self.sender.send_request(request).await {
            Err(error) => return SocketError::raise_request_failed(url, error),
            Ok(value) => value,
        };

        let status: StatusCode = response.status();
        let response: SocketResponse = SocketResponse::new(url, response, self.connection);

        if !status.is_success() {
            return SocketError::raise_status_failed(status, response);
        }

        Ok(response)
    }

    pub async fn get(self, url: &str) -> SocketResult<SocketResponse> {
        let request = Request::builder()
            .uri(url)
            .method("GET")
            .header("Host", "localhost")
            .body(Full::new(Bytes::new()));

        let request: Request<Full<Bytes>> = match request {
            Err(error) => return SocketError::raise_builder_failed(url, error),
            Ok(value) => value,
        };

        self.execute(url, request).await
    }

    pub async fn put(self, url: &str) -> SocketResult<SocketResponse> {
        let request = Request::builder()
            .uri(url)
            .method("PUT")
            .header("Host", "localhost")
            .body(Full::new(Bytes::new()));

        let request: Request<Full<Bytes>> = match request {
            Err(error) => return SocketError::raise_builder_failed(url, error),
            Ok(value) => value,
        };

        self.execute(url, request).await
    }

    pub async fn post(self, url: &str, body: Option<Value>) -> SocketResult<SocketResponse> {
        let request = Request::builder()
            .uri(url)
            .method("POST")
            .header("Host", "localhost")
            .header("Content-Type", "application/json");

        let request = match body {
            None => request.body(Full::new(Bytes::new())),
            Some(value) => request.body(Full::new(Bytes::from(value.to_string()))),
        };

        let request: Request<Full<Bytes>> = match request {
            Err(error) => return SocketError::raise_builder_failed(url, error),
            Ok(value) => value,
        };

        self.execute(url, request).await
    }

    pub async fn delete(self, url: &str) -> SocketResult<SocketResponse> {
        let request = Request::builder()
            .uri(url)
            .method("DELETE")
            .header("Host", "localhost")
            .body(Full::new(Bytes::new()));

        let request: Request<Full<Bytes>> = match request {
            Err(error) => return SocketError::raise_builder_failed(url, error),
            Ok(value) => value,
        };

        self.execute(url, request).await
    }
}
