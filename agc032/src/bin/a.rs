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
        mut b: [Usize1; n],
    }
    let mut result = vec![];
    'outer: for _ in 0..n {
        for i in (0..b.len()).rev() {
            if b[i] == i {
                b.remove(i);
                result.push(i);
                continue 'outer;
            }
        }
        println!("-1");
        return;
    }
    result.reverse();
    for &x in &result {
        println!("{}", x + 1);
    }
}
