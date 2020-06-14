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
        n: usize,
        tx: isize,
        ty: isize,
        xy: [(isize, isize); n],
    }
    let objects = xy.into_iter().collect::<HashSet<_>>();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    let dirs = [(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];
    while let Some((x, y, d)) = queue.pop_front() {
        if visited.contains(&(x, y)) || d > 500 * 500 {
            continue;
        }
        if x == tx && y == ty {
            println!("{}", d);
            return;
        }
        visited.insert((x, y));
        for &(dx, dy) in &dirs {
            let x2 = x + dx;
            let y2 = y + dy;
            if !objects.contains(&(x2, y2)) && x2.abs() < 205 && y2.abs() < 205 {
                queue.push_back((x2, y2, d + 1));
            }
        }
    }
    println!("-1");
}
