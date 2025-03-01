use std::path::PathBuf;
use axum::extract::Query;
use crate::routes::filters;
use crate::{Files, PathRequest};
use axum::response::Html;
use rinja::Template;
use serde::Deserialize;
use crate::routes::file_list::get_files;

#[derive(Deserialize)]
pub struct IndexQuery {
    #[serde(rename = "path")]
    pub directory: Option<PathBuf>,
}

#[axum::debug_handler]
pub async fn get_index(path: PathRequest) -> Result<Html<String>, ()> {
    #[derive(Template, Debug)]
    #[template(path = "index.html")]
    struct Tmpl {
        lang: String,
        files: Files,
        descending: bool,
    }

    let files = get_files(path.directory).await;

    let template = Tmpl {
        lang: "en".to_string(),
        files,
        descending: true,
    };

    Ok(Html(template.render().unwrap()))
}
