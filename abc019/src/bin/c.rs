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

    let mut set = HashSet::new();
    for &ai in &a {
        let mut x = ai;
        while x % 2 == 0 {
            x /= 2;
        }
        set.insert(x);
    }
    println!("{}", set.len());
}
