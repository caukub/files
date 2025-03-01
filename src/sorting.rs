use crate::Files;
use serde::Deserialize;
use std::cmp::Ordering;

pub struct FileSorter {
    files: Files,
}

impl FileSorter {
    pub fn new(files: Files) -> Self {
        Self { files }
    }

    pub fn sort(mut self, sort_type: &SortType) -> Files {
        match sort_type {
            SortType::Default(_) => {
                self.files
                    .sort_by(|a, b| match (a.is_directory, b.is_directory) {
                        (true, false) => Ordering::Less,
                        (false, true) => Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    });
            }
            SortType::Name(order) => {
                self.files.sort_by(|a, b| match order {
                    SortOrder::Ascending => a.name.cmp(&b.name),
                    SortOrder::Descending => b.name.cmp(&a.name),
                });
            }
            SortType::Size(order) => {
                self.files.sort_by(|a, b| match order {
                    SortOrder::Ascending => b.size.cmp(&a.size),
                    SortOrder::Descending => a.size.cmp(&b.size),
                });
            }
            SortType::Modified(order) => {
                self.files.sort_by(|a, b| match order {
                    SortOrder::Ascending => b.modified.cmp(&a.modified),
                    SortOrder::Descending => a.modified.cmp(&b.modified),
                });
            }
        }
        self.files
    }
}

pub fn deserialize_sorting<'de, D>(deserializer: D) -> Result<SortType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    let parts: Vec<&str> = s.split('.').collect();

    match parts.as_slice() {
        ["default", "unix"] => Ok(SortType::Default(SortTypeDefault::Unix)),
        ["default", "windows"] => Ok(SortType::Default(SortTypeDefault::Windows)),
        ["name", "ascending"] => Ok(SortType::Name(SortOrder::Ascending)),
        ["name", "descending"] => Ok(SortType::Name(SortOrder::Descending)),
        ["size", "ascending"] => Ok(SortType::Size(SortOrder::Ascending)),
        ["size", "descending"] => Ok(SortType::Size(SortOrder::Descending)),
        ["modified", "ascending"] => Ok(SortType::Modified(SortOrder::Ascending)),
        ["modified", "descending"] => Ok(SortType::Modified(SortOrder::Descending)),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid sort format: {}",
            s
        ))),
    }
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortType {
    Default(SortTypeDefault),
    Name(SortOrder),
    Size(SortOrder),
    Modified(SortOrder),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortTypeDefault {
    Unix,
    Windows,
}

impl Default for SortType {
    fn default() -> Self {
        if cfg!(windows) {
            SortType::Default(SortTypeDefault::Windows)
        } else {
            SortType::Default(SortTypeDefault::Unix)
        }
    }
}
