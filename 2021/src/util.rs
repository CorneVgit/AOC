use std::str::FromStr;

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    let file_path = format!(
        "D:/programming/Advent of Code/2021/aoc2021/data/{}.txt",
        file_name
    );

    std::fs::read_to_string(file_path)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
