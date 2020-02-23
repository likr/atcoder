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
        s: Chars,
    }
    let n = s.len();

    let mut result = 0;
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        if s[l] == 'x' && s[r] == 'x' {
            l += 1;
            r -= 1;
        } else if s[l] == 'x' {
            result += 1;
            l += 1;
        } else if s[r] == 'x' {
            result += 1;
            r -= 1;
        } else {
            if s[l] != s[r] {
                println!("-1");
                return;
            }
            l += 1;
            r -= 1;
        }
    }
    println!("{}", result);
}
