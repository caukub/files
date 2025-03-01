pub mod delete;
pub mod file_list;
pub mod foo;
pub mod index;
pub mod video;

pub mod filters {
    use humansize::DECIMAL;
    use std::str::FromStr;

    pub fn format_size<T: std::fmt::Display>(s: T) -> ::rinja::Result<String> {
        let size = usize::from_str(&s.to_string()).unwrap();
        Ok(humansize::format_size(size, DECIMAL))
    }
}
