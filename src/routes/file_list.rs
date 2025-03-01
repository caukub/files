use crate::routes::filters;
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
    sorting: Sorting,
}

impl Default for Sorting {
    fn default() -> Self {
        Sorting::Default(DefaultSortType::Unix)
    }
}

pub async fn get_file_list(Query(query): Query<GetFileQuery>, path: PathRequest) -> Html<String> {
    #[derive(Template, Debug)]
    #[template(path = "file-list.html")]
    struct Tmpl {
        files: Files,
        sorting: Sorting,
        path_request: PathRequest,
    }

    let sorting = query.sorting.clone();

    let template = Tmpl {
        files: sort(get_files(&path.directory).await, query.sorting),
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
fn deserialize_sorting<'de, D>(deserializer: D) -> Result<Sorting, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    let parts: Vec<&str> = s.split('.').collect();

    match parts.as_slice() {
        ["default", "unix"] => Ok(Sorting::Default(DefaultSortType::Unix)),
        ["default", "windows"] => Ok(Sorting::Default(DefaultSortType::Windows)),
        ["name", "ascending"] => Ok(Sorting::Name(SortingType::Ascending)),
        ["name", "descending"] => Ok(Sorting::Name(SortingType::Descending)),
        ["size", "ascending"] => Ok(Sorting::Size(SortingType::Ascending)),
        ["size", "descending"] => Ok(Sorting::Size(SortingType::Descending)),
        ["modified", "ascending"] => Ok(Sorting::Modified(SortingType::Ascending)),
        ["modified", "descending"] => Ok(Sorting::Modified(SortingType::Descending)),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid sort format: {}",
            s
        ))),
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortingType {
    Ascending,
    Descending,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sorting {
    Default(DefaultSortType),
    Name(SortingType),
    Size(SortingType),
    Modified(SortingType),
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DefaultSortType {
    Unix,
    Windows,
}

pub fn sort(mut files: Files, sorting: Sorting) -> Files {
    match sorting {
        Sorting::Name(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => a.name.cmp(&b.name),
                SortingType::Descending => b.name.cmp(&a.name),
            });
        }
        Sorting::Size(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => b.size.cmp(&a.size),
                SortingType::Descending => a.size.cmp(&b.size),
            });
        }
        Sorting::Modified(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => b.modified.cmp(&a.modified),
                SortingType::Descending => a.modified.cmp(&b.modified),
            });
        }
        _ => {}
    }
    files
}
