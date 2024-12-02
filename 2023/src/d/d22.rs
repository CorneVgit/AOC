use euclid::{
    box3d,
    default::{Box3D, Translation3D},
};
use itertools::Itertools;

use crate::util::read_all;

fn get_bricks() -> Vec<Box3D<i64>> {
    let result_values = read_all::<String>("input_22_sample");

    result_values
        .into_iter()
        .map(unwrap_infallible::UnwrapInfallible::unwrap_infallible)
        .map(|pair| {
            let (p1, p2) = pair.split_once('~').unwrap();
            let (min_x, min_y, min_z) = p1
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (max_x, max_y, max_z) = p2
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();

            box3d(min_x, min_y, min_z, max_x, max_y, max_z)
        })
        .collect_vec()
}

#[must_use]
pub fn d22() -> (usize, usize) {
    let mut bricks = get_bricks();

    let down_by_1 = Translation3D::new(0, 0, -1);

    // for i in 0..bricks.len() {
    //     while !bricks[..]
    //         .iter()
    //         .any(|b|{
    //             b.intersects(&bricks[i].translate(down_by_1.into())) && *b != bricks[i]
    //         })
    //         && bricks[i].min.z > 1
    //     {
    //         bricks[i] = bricks[i].translate(down_by_1.into());
    //     }
    // }

    // println!("{bricks:?}");

    // for pair in bricks.iter().enumerate().permutations(2) {
    //     if let [b1, b2] = pair[..2] {
    //         if b1.1.intersects(&b2.1.translate(down_by_1.into())) {
    //             println!("{}, {}: true", b1.0, b2.0);
    //         }
    //     }
    // }

    println!("{:?}", bricks[0]);
    println!("{:?}", bricks[1]);
    println!("{:?}", bricks[1].translate(down_by_1.into()));
    println!("{}", bricks[0].intersects(&bricks[1]));
    println!("{}", bricks[0].intersects(&bricks[1].translate(down_by_1.into())));

    (0, 0)
}
