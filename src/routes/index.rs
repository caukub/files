use crate::routes::file_list::get_files;
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
struct Tmpl {
    lang: String,
    files: Files,
    sorting: SortType,
    path_request: PathRequest,
}

pub async fn get_index(
    path_request: PathRequest,
    Query(query): Query<GetIndexQuery>,
) -> Result<Html<String>, ()> {
    let files = get_files(&path_request.directory).await;
    let files = FileSorter::new(files).sort(&query.sorting);

    let template = Tmpl {
        lang: "en".to_string(),
        files,
        sorting: query.sorting,
        path_request,
    };

    Ok(Html(template.render().unwrap()))
}
