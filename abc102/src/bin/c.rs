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
        a: [isize; n],
    }
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, ai)| ai - (i as isize + 1))
        .collect::<Vec<_>>();
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(ai) += 1;
    }
    let mut a = a
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    a.sort();

    let mut l = a.iter().min().unwrap();
    let mut r = a.iter().max().unwrap();
    while r - l > 1 {
        let m = (l + r) / 2;
    }
    println!("{}", l);
}
