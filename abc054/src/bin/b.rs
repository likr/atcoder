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
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    }
    for i in 0..=n - m {
        'outer: for j in 0..=n - m {
            for k in 0..m {
                for l in 0..m {
                    if a[i + k][j + l] != b[k][l] {
                        continue 'outer;
                    }
                }
            }
            println!("Yes");
            return;
        }
    }
    println!("No");
}
