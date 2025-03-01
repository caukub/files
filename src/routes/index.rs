use crate::error::AppError;
use crate::routes::file_list::read_directory;
use crate::routes::filters;
use crate::sorting::{FileSorter, SortOrder, SortType, deserialize_sorting};
use crate::{Files, PathRequest};
use axum::extract::Query;
use axum::response::Html;
use rinja::Template;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetIndexQuery {
    #[serde(default, deserialize_with = "deserialize_sorting")]
    sorting: SortType,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    lang: String,
    files: Files,
    sorting: SortType,
    path_request: PathRequest,
}

pub async fn get_index(
    path_request: PathRequest,
    Query(query): Query<GetIndexQuery>,
) -> Result<Html<String>, AppError> {
    let files = read_directory(&path_request.directory)
        .await
        .map_err(|_err| AppError::ReadingDirectory)?;
    let files = FileSorter::new(files).sort(&query.sorting);

    let template = IndexTemplate {
        lang: "en".to_string(),
        files,
        sorting: query.sorting,
        path_request,
    };

    Ok(Html(template.render().map_err(|_| AppError::Foo)?))
}
