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
        h: isize,
        w: isize,
        n: usize,
        ab: [(Isize1, Isize1); n],
    }
    let mut count = vec![0; 10];
    let mut points = HashSet::new();
    for &(ai, bi) in &ab {
        points.insert((ai, bi));
    }
    let mut visited = HashSet::new();
    for &(ai, bi) in &ab {
        for &c in &[-1, 0, 1] {
            for &d in &[-1, 0, 1] {
                let cx = ai + c;
                let cy = bi + d;
                if 0 < cx && cx < h - 1 && 0 < cy && cy < w - 1 && !visited.contains(&(cx, cy)) {
                    let mut g = 0;
                    for &e in &[-1, 0, 1] {
                        for &f in &[-1, 0, 1] {
                            let x = cx + e;
                            let y = cy + f;
                            if points.contains(&(x, y)) {
                                g += 1;
                            }
                        }
                    }
                    if g > 0 {
                        count[g] += 1;
                    }
                    visited.insert((cx, cy));
                }
            }
        }
    }
    println!("{}", (h - 2) * (w - 2) - count.iter().sum::<isize>());
    for i in 1..10 {
        println!("{}", count[i]);
    }
}
