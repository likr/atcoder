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
    let mut odd = 0;
    let mut four = 0;
    for &ai in &a {
        if ai % 2 == 1 {
            odd += 1;
        }
        if ai % 4 == 0 {
            four += 1;
        }
    }
    if (n % 2 == 0 && four >= odd) || (n % 2 == 1 && four + 1 >= odd) {
        println!("Yes");
    } else {
        println!("No");
    }
}
