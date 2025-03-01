use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Couldn't read directory")]
    ReadingDirectory,
    #[error("Couldn't convert")]
    NameConversion,
    #[error("foo barz baz")]
    Foo,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Response::default()
    }
}
