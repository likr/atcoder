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
        _n: usize,
        a: usize,
        b: usize,
        k: usize,
        p: [usize; k],
    }
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    for &pi in &p {
        set.insert(pi);
    }
    if set.len() == k + 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
