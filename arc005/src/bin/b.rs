use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        mut x: Isize1,
        mut y: Isize1,
        w: String,
        c: [Chars; 9],
    }
    let (mut dx, mut dy) = match &*w {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        "RU" => (1, -1),
        "RD" => (1, 1),
        "LU" => (-1, -1),
        "LD" => (-1, 1),
        _ => (0, 0),
    };
    for _ in 0..4 {
        print!("{}", c[y as usize][x as usize]);
        if (dx < 0 && x == 0) || (dx > 0 && x == 8) {
            dx = -dx;
        }
        if (dy < 0 && y == 0) || (dy > 0 && y == 8) {
            dy = -dy;
        }
        x += dx;
        y += dy;
    }
    println!("");
}
