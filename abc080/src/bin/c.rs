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
    let m = 10;
    input! {
        n: usize,
        f: [[usize; m]; n],
        p: [[isize; m + 1]; n],
    }
    let mut result = -(INF as isize);
    for x in 1..1024 {
        let mut s = 0;
        for i in 0..n {
            let mut c = 0;
            for j in 0..m {
                if f[i][j] == 1 && x & 1 << j > 0 {
                    c += 1;
                }
            }
            s += p[i][c];
        }
        // eprintln!("{:010b} {}", x, s);
        result = max(result, s);
    }
    println!("{}", result);
}
