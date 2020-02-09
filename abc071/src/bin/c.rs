use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      a: [usize; n],
    }
    let mut count = HashMap::new();
    for &ai in &a {
        if !count.contains_key(&ai) {
            count.insert(ai, 0);
        }
        count.insert(ai, count[&ai] + 1);
    }
    let mut edges = count.keys().filter(|l| count[&l] >= 2).collect::<Vec<_>>();
    edges.sort();
    edges.reverse();
    if edges.len() == 0 || (edges.len() == 1 && count[&edges[0]] < 4) {
        println!("0");
    } else {
        if count[&edges[0]] >= 4 {
            println!("{}", edges[0] * edges[0]);
        } else {
            println!("{}", edges[0] * edges[1]);
        }
    }
}
