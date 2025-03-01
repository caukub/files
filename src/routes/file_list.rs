use crate::routes::filters;
use crate::sorting::{FileSorter, SortOrder, SortType, deserialize_sorting};
use crate::{File, Files, PathRequest};
use axum::extract::Query;
use axum::response::Html;
use rinja::Template;
use serde::Deserialize;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct GetFileQuery {
    #[serde(deserialize_with = "deserialize_sorting")]
    #[serde(default)]
    sorting: SortType,
}

pub async fn get_file_list(Query(query): Query<GetFileQuery>, path: PathRequest) -> Html<String> {
    #[derive(Template, Debug)]
    #[template(path = "file-list.html")]
    struct Tmpl {
        files: Files,
        sorting: SortType,
        path_request: PathRequest,
    }

    let sorting = query.sorting.clone();

    let files = get_files(&path.directory).await;
    let files = FileSorter::new(files).sort(&query.sorting);
    let template = Tmpl {
        files,
        sorting,
        path_request: path,
    };

    Html(template.render().unwrap())
}

pub async fn get_files(directory: impl AsRef<Path>) -> Files {
    let mut files: Files = Vec::new();

    let mut entries = tokio::fs::read_dir(directory).await.unwrap();

    while let Some(entry) = entries.next_entry().await.unwrap() {
        let metadata = entry.metadata().await.unwrap();
        let path = if entry.path().parent().unwrap().to_str() == Some(".") {
            ".".to_string()
        } else {
            entry.path().parent().unwrap().to_str().unwrap().to_string()
        };

        let file = File {
            name: entry
                .file_name()
                .to_os_string()
                .to_string_lossy()
                .to_string(),
            modified: metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: metadata.len(),
            is_directory: entry.file_type().await.unwrap().is_dir(),
            path,
        };

        files.push(file);
    }

    files
}
