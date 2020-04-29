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
        a: [usize; n],
    }
    let s = a.iter().map(|&ai| ai).collect::<HashSet<_>>();
    let mut s = s.into_iter().collect::<Vec<_>>();
    s.sort();
    let mut b = HashMap::new();
    for i in 0..s.len() {
        let ai = s[i];
        b.insert(ai, i);
    }
    for &ai in &a {
        println!("{}", b[&ai]);
    }
}
