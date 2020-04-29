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
        x: Chars,
    }
    let n = x.len();
    let mut i = 0;
    while i < n {
        if i + 1 < n && x[i] == 'c' && x[i + 1] == 'h' {
            i += 2;
        } else if x[i] == 'o' || x[i] == 'k' || x[i] == 'u' {
            i += 1;
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
