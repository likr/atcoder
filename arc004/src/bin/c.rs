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
        xy: String,
    }
    let xy = xy
        .split('/')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let x = xy[0];
    let y = xy[1];
    let mut count = 0;
    let n_start = if 2 * x >= y { (2 * x - y) / y } else { 1 };
    for n in n_start.. {
        if y * (n + 1) >= 2 * x {
            if y * (n + 1) > 2 * (x + y) {
                break;
            }
            if n * (y * (n + 1) - 2 * x) % (2 * y) == 0 && n * (y * (n + 1) - 2 * x) / (2 * y) > 0 {
                count += 1;
                println!("{} {}", n, n * (y * (n + 1) - 2 * x) / (2 * y));
            }
        }
    }
    if count == 0 {
        println!("Impossible");
    }
}
