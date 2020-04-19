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
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut c = 0;
    let mut d = 0;
    for i in 0..n {
        if s[i] == 'a' && c < a + b {
            c += 1;
            println!("Yes");
        } else if s[i] == 'b' && c < a + b && d < b {
            c += 1;
            d += 1;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
