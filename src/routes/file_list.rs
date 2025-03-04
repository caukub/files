use crate::error::AppError;
use crate::routes::filters;
use crate::sorting::{FileSorter, SortOrder, SortType, deserialize_sorting};
use crate::{File, Files, PathRequest};
use axum::extract::Query;
use axum::response::Html;
use chrono::DateTime;
use rinja::Template;
use serde::Deserialize;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Deserialize)]
pub struct FileListQuery {
    #[serde(default, deserialize_with = "deserialize_sorting")]
    sorting: SortType,
}

#[derive(Template)]
#[template(path = "file-list.html")]
struct FileListPath {
    files: Files,
    sorting: SortType,
    path_request: PathRequest,
}

pub async fn get_file_list(
    Query(query): Query<FileListQuery>,
    path: PathRequest,
) -> Result<Html<String>, AppError> {
    let sorting = query.sorting;

    let files = read_directory(&path.directory)
        .await
        .map_err(|_| AppError::ReadingDirectory)?;
    let files = FileSorter::new(files).sort(&sorting);

    let template = FileListPath {
        files,
        sorting,
        path_request: path,
    };

    Ok(Html(template.render().map_err(|_| AppError::Foo)?))
}

pub async fn read_directory(directory: impl AsRef<Path>) -> Result<Files, AppError> {
    let mut files: Files = Vec::new();

    let mut entries = tokio::fs::read_dir(directory)
        .await
        .map_err(|_err| AppError::ReadingDirectory)?;

    while let Some(entry) = entries.next_entry().await.map_err(|_| AppError::Foo)? {
        let metadata = entry.metadata().await.map_err(|_| AppError::Foo)?;
        let path = if entry.path().parent().unwrap().to_str() == Some(".") {
            ".".to_string()
        } else {
            entry.path().parent().unwrap().to_str().unwrap().to_string()
        };

        let modified = metadata
            .modified()
            .map_err(|_| AppError::Foo)?
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::Foo)?
            .as_secs();

        let name = entry
            .file_name()
            .into_string()
            .map_err(|_err| AppError::NameConversion)?;

        let size = metadata.len();

        let is_directory = entry.file_type().await.map_err(|_| AppError::Foo)?.is_dir();

        let date_modified = DateTime::from_timestamp(modified as i64, 0).unwrap();

        let file = File {
            name,
            modified,
            size,
            is_directory,
            path,
            date_modified,
        };

        files.push(file);
    }

    Ok(files)
}
