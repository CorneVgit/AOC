use std::str::FromStr;

// use ascii::{AsciiString, IntoAsciiString};

#[must_use]
pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    let file_path = format!("data\\{file_name}.txt");

    std::fs::read_to_string(file_path)
        .expect("file not found!")
        .lines()
        .map(str::parse)
        .collect()
}

#[must_use]
pub fn read_all_vec_unsafe<T: FromStr>(file_name: &str) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file_path = format!("data\\{file_name}.txt");

    std::fs::read_to_string(file_path)
        .expect("file not found!")
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

#[must_use]
pub fn read_single_string(file_name: &str) -> String {
    let file_path = format!("data\\{file_name}.txt");

    std::fs::read_to_string(file_path).expect("file not found!")
}

// #[must_use] pub fn read_single_ascii_string(file_name: &str) -> AsciiString {
//     read_single_string(file_name).trim().into_ascii_string().unwrap()
// }

// fn get_next_position<T: nalgebra::Scalar + num::PrimInt + num::Signed + std::ops::AddAssign>(
//     current_position: &Point2<T>,
//     direction: &Direction,
// ) -> Point2<T> {
//     match direction {
//         Direction::North => (current_position.coords
//             + Point2::new(T::from(0).unwrap(), T::from(-1).unwrap()).coords)
//             .into(),
//         Direction::East => (current_position.coords
//             + Point2::new(T::from(1).unwrap(), T::from(0).unwrap()).coords)
//             .into(),
//         Direction::South => (current_position.coords
//             + Point2::new(T::from(0).unwrap(), T::from(1).unwrap()).coords)
//             .into(),
//         Direction::West => (current_position.coords
//             + Point2::new(T::from(-1).unwrap(), T::from(0).unwrap()).coords)
//             .into(),
//     }
// }
