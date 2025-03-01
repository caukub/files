use std::cmp::Ordering;
use axum::extract::Query;
use crate::routes::file_list::{DefaultSortType, Sorting, SortingType, get_files, deserialize_sorting};
use crate::routes::filters;
use crate::{Files, PathRequest};
use axum::response::Html;
use rinja::Template;
use serde::Deserialize;
use crate::sorting::FileSorter;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct GetIndexQuery {
    #[serde(deserialize_with = "deserialize_sorting")]
    #[serde(default)]
    sorting: Sorting,
}

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct Tmpl {
    lang: String,
    files: Files,
    sorting: Sorting,
    path_request: PathRequest,
}

pub async fn get_index(path_request: PathRequest, Query(query): Query<GetIndexQuery>) -> Result<Html<String>, ()> {
    let files = get_files(&path_request.directory).await;
    let files = FileSorter::new(files).sort(&query.sorting);

    let template = Tmpl {
        lang: "en".to_string(),
        files,
        sorting: Sorting::Default(DefaultSortType::Unix),
        path_request,
    };

    Ok(Html(template.render().unwrap()))
}