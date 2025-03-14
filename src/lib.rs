use axum::extract::{FromRequestParts, Query};
use axum::http::StatusCode;
use axum::http::request::Parts;
use chrono::{DateTime, Utc};
use std::path::PathBuf;

pub mod configuration;
pub mod error;
pub mod routes;
pub mod sorting;
pub mod tracing;

pub type Files = Vec<File>;

pub struct File {
    pub name: String,
    pub modified: u64,
    pub size: u64,
    pub path: String,
    pub is_directory: bool,
    pub date_modified: DateTime<Utc>,
}

pub struct PathRequest {
    pub directory: PathBuf,
    pub file: Option<PathBuf>,
    pub full_path: PathBuf,
}

#[derive(serde::Deserialize)]
struct FilePath {
    #[serde(rename = "path")]
    directory: PathBuf,
    file: Option<PathBuf>,
}

impl<S> FromRequestParts<S> for PathRequest
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<FilePath>::from_request_parts(parts, state).await;

        let Ok(mut query) = query else {
            return Ok(PathRequest {
                directory: PathBuf::from("."),
                file: None,
                full_path: PathBuf::from("."),
            });
        };

        if query.directory == PathBuf::from("/") {
            query.directory = PathBuf::from(".")
        }

        Ok(PathRequest {
            file: query.file.clone(),
            directory: query.directory.clone(),
            full_path: query
                .directory
                .join(query.file.clone().unwrap_or(PathBuf::from(""))),
        })
    }
}
