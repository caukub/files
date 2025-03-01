use std::cmp::Ordering;
use crate::Files;
use crate::routes::file_list::{Sorting, SortingType};

pub struct FileSorter {
    files: Files,
}

impl FileSorter {
    pub fn new(files: Files) -> Self {
        Self { files }
    }

    pub fn sort(mut self, sort_type: &Sorting) -> Files {
        match sort_type {
            Sorting::Default(_) => {
                self.files.sort_by(|a, b| {
                    match (a.is_directory, b.is_directory) {
                        (true, false) => Ordering::Less,
                        (false, true) => Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                });
            }
            Sorting::Name(order) => {
                self.files.sort_by(|a, b| match order {
                    SortingType::Ascending => a.name.cmp(&b.name),
                    SortingType::Descending => b.name.cmp(&a.name),
                });
            }
            Sorting::Size(order) => {
                self.files.sort_by(|a, b| match order {
                    SortingType::Ascending => b.size.cmp(&a.size),
                    SortingType::Descending => a.size.cmp(&b.size),
                });
            }
            Sorting::Modified(order) => {
                self.files.sort_by(|a, b| match order {
                    SortingType::Ascending => b.modified.cmp(&a.modified),
                    SortingType::Descending => a.modified.cmp(&b.modified),
                });
            }
        }
        self.files
    }
}