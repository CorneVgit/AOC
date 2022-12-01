use crate::util::read_all;

#[derive(Debug, Clone, Copy)]
struct Point {
    value: u32,
    x: usize,
    y: usize,
}

impl Point {
    fn new(value: u32, x: usize, y: usize) -> Self {
        Self { value, x, y }
    }
}

fn d9values() -> Vec<Vec<Point>> {
    let result_values = read_all::<String>("input_9");
    let mut values: Vec<Vec<Point>> = Vec::new();

    for (i, result_value) in result_values.into_iter().enumerate() {
        let value = match result_value {
            Ok(x) => x,
            _ => continue,
        };

        let row: Vec<Point> = value
            .chars()
            .into_iter()
            .enumerate()
            .map(|(j, c)| Point::new(char::to_digit(c, 10).unwrap(), i, j))
            .collect();

        values.push(row);
    }

    values
}

pub fn d9a() {
    let values = d9values();

    let low_points = get_low_points(&values);

    println!(
        "{}",
        low_points.into_iter().map(|p| p.value + 1).sum::<u32>()
    );
}

pub fn d9b() {
    let values = d9values();

    let low_points = get_low_points(&values);

    let row_count = values.len();
    let col_count = values.first().unwrap().len();

    let mut basin_sizes: Vec<usize> = Vec::new();

    let mut marked = vec![vec![false; col_count]; row_count];

    for low_point in &low_points {
        basin_sizes.push(calculate_basin_size(&values, low_point, &mut marked));
    }

    basin_sizes.sort();

    println!(
        "{:?}",
        &basin_sizes[basin_sizes.len() - 3..basin_sizes.len()]
            .iter()
            .product::<usize>()
    );
}

fn calculate_basin_size(values: &[Vec<Point>], point: &Point, marked: &mut [Vec<bool>]) -> usize {
    let row_count = values.len();
    let col_count = values.first().unwrap().len();

    let mut size: usize = 1;

    let m = marked.get_mut(point.x).unwrap().get_mut(point.y).unwrap();

    if point.value == 9 || *m {
        return 0;
    }

    *m = true;

    if point.x < row_count {
        if let Some(v) = get_value(values, point.x + 1, point.y) {
            if v.value > point.value {
                size += calculate_basin_size(values, v, marked);
            }
        }
    }
    if point.x > 0 {
        if let Some(v) = get_value(values, point.x - 1, point.y) {
            if v.value > point.value {
                size += calculate_basin_size(values, v, marked);
            }
        }
    }
    if point.y < col_count {
        if let Some(v) = get_value(values, point.x, point.y + 1) {
            if v.value > point.value {
                size += calculate_basin_size(values, v, marked);
            }
        }
    }
    if point.y > 0 {
        if let Some(v) = get_value(values, point.x, point.y - 1) {
            if v.value > point.value {
                size += calculate_basin_size(values, v, marked);
            }
        }
    }

    size
}

fn get_low_points(values: &[Vec<Point>]) -> Vec<Point> {
    let row_count = values.len();
    let col_count = values.first().unwrap().len();

    let mut low_points: Vec<Point> = Vec::new();

    for x in 0..row_count {
        for y in 0..col_count {
            let cur = values.get(x).unwrap().get(y).unwrap();

            if x < row_count {
                if let Some(v) = get_value(values, x + 1, y) {
                    if v.value <= cur.value {
                        continue;
                    }
                }
            }
            if x > 0 {
                if let Some(v) = get_value(values, x - 1, y) {
                    if v.value <= cur.value {
                        continue;
                    }
                }
            }
            if y < col_count {
                if let Some(v) = get_value(values, x, y + 1) {
                    if v.value <= cur.value {
                        continue;
                    }
                }
            }
            if y > 0 {
                if let Some(v) = get_value(values, x, y - 1) {
                    if v.value <= cur.value {
                        continue;
                    }
                }
            }

            low_points.push(*cur)
        }
    }

    low_points
}

fn get_value(values: &[Vec<Point>], x: usize, y: usize) -> Option<&Point> {
    match values.get(x) {
        Some(row) => row.get(y),
        None => None,
    }
}
