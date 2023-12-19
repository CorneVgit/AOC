use std::str::FromStr;

use ascii::{AsciiString, IntoAsciiString};

#[must_use] pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    let file_path = format!("data\\{file_name}.txt");

    std::fs::read_to_string(file_path)
        .expect("file not found!")
        .lines()
        .map(str::parse)
        .collect()
}

#[must_use] pub fn read_single_string(file_name: &str) -> String {
    let file_path = format!("data\\{file_name}.txt");

    std::fs::read_to_string(file_path).expect("file not found!")
}

#[must_use] pub fn read_single_ascii_string(file_name: &str) -> AsciiString {
    read_single_string(file_name).trim().into_ascii_string().unwrap()
}
