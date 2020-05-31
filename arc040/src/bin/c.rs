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
        mut s: [Chars; n],
    }
    let mut result = 0;
    for i in 0..n {
        if let Some(c) = s[i].iter().rposition(|&x| x == '.') {
            for j in 0..=c {
                s[i][j] = 'o';
            }
            if i + 1 < n {
                for j in c..n {
                    s[i + 1][j] = 'o'
                }
            }
            result += 1;
        }
    }
    println!("{}", result);
}
