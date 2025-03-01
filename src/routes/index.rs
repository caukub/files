use crate::routes::file_list::{DefaultSortType, Sorting, SortingType, get_files};
use crate::routes::filters;
use crate::{Files, PathRequest};
use axum::response::Html;
use rinja::Template;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct IndexQuery {
    #[serde(rename = "path")]
    pub directory: Option<PathBuf>,
}

pub async fn get_index(path_request: PathRequest) -> Result<Html<String>, ()> {
    #[derive(Template, Debug)]
    #[template(path = "index.html")]
    struct Tmpl {
        lang: String,
        files: Files,
        sorting: Sorting,
        path_request: PathRequest,
    }

    let files = get_files(&path_request.directory).await;

    let template = Tmpl {
        lang: "en".to_string(),
        files,
        sorting: Sorting::Default(DefaultSortType::Unix),
        path_request,
    };

    Ok(Html(template.render().unwrap()))
}
