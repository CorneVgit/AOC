use std::collections::HashSet;

use nalgebra::Point2;

use crate::util::read_single_string;

fn values() -> Vec<char> {
    read_single_string("input_17").chars().collect()
}

pub fn d17() -> (usize, usize) {
    let directions = values();

    let rock_shapes = vec![
        HashSet::from([
            Point2::<usize>::new(2, 0),
            Point2::<usize>::new(3, 0),
            Point2::<usize>::new(4, 0),
            Point2::<usize>::new(5, 0),
        ]),
        HashSet::from([
            Point2::<usize>::new(3, 0),
            Point2::<usize>::new(2, 1),
            Point2::<usize>::new(3, 1),
            Point2::<usize>::new(4, 1),
            Point2::<usize>::new(3, 2),
        ]),
        HashSet::from([
            Point2::<usize>::new(2, 0),
            Point2::<usize>::new(3, 0),
            Point2::<usize>::new(4, 0),
            Point2::<usize>::new(4, 1),
            Point2::<usize>::new(4, 2),
        ]),
        HashSet::from([
            Point2::<usize>::new(2, 0),
            Point2::<usize>::new(2, 1),
            Point2::<usize>::new(2, 2),
            Point2::<usize>::new(2, 3),
        ]),
        HashSet::from([
            Point2::<usize>::new(2, 0),
            Point2::<usize>::new(3, 0),
            Point2::<usize>::new(2, 1),
            Point2::<usize>::new(3, 1),
        ]),
    ];

    let mut tower = HashSet::from([
        Point2::<usize>::new(0, 0),
        Point2::<usize>::new(1, 0),
        Point2::<usize>::new(2, 0),
        Point2::<usize>::new(3, 0),
        Point2::<usize>::new(4, 0),
        Point2::<usize>::new(5, 0),
        Point2::<usize>::new(6, 0),
    ]);

    let mut reps = HashSet::<(usize, usize)>::new();

    let mut direction_index = 0;
    for rock_count in 0..100000 {
        let tower_max = tower.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

        let rep = (
            direction_index % directions.len(),
            rock_count % rock_shapes.len(),
        );

        match reps.get(&rep) {
            Some(_) => {
                println!("yep, {rep:?}, {tower_max}");
                reps.clear();
                reps.insert(rep);
            }
            None => {
                reps.insert(rep);
            }
        }

        let mut rock_shape: HashSet<Point2<usize>> = rock_shapes[rock_count % rock_shapes.len()]
            .iter()
            .map(|r| Point2::<usize>::new(r.x, r.y + tower_max + 4))
            .collect();

        while !tower.iter().any(|r| rock_shape.contains(r)) {
            match directions[direction_index % directions.len()] {
                '>' => {
                    if !rock_shape.iter().any(|p| p.x == 6) {
                        rock_shape = rock_shape
                            .iter()
                            .map(|r| Point2::<usize>::new(r.x + 1, r.y))
                            .collect();

                        if tower.iter().any(|r| rock_shape.contains(r)) {
                            rock_shape = rock_shape
                                .iter()
                                .map(|r| Point2::<usize>::new(r.x - 1, r.y))
                                .collect();
                        }
                    }
                }
                '<' => {
                    if !rock_shape.iter().any(|p| p.x == 0) {
                        rock_shape = rock_shape
                            .iter()
                            .map(|r| Point2::<usize>::new(r.x - 1, r.y))
                            .collect();

                        if tower.iter().any(|r| rock_shape.contains(r)) {
                            rock_shape = rock_shape
                                .iter()
                                .map(|r| Point2::<usize>::new(r.x + 1, r.y))
                                .collect();
                        }
                    }
                }
                _ => {
                    direction_index += 1;
                    continue;
                }
            };

            direction_index += 1;

            rock_shape = rock_shape
                .iter()
                .map(|r| Point2::<usize>::new(r.x, r.y - 1))
                .collect();
        }

        rock_shape.iter().for_each(|r| {
            tower.insert(Point2::<usize>::new(r.x, r.y + 1));
        });
    }

    let r1 = tower.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let r2 = 0;

    (r1, r2)
}
