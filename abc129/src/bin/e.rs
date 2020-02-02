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
      l: Chars,
    }
    let n = l.len();
    let mut result = 0;
    let mut ones = 0;
    let mut pow2 = vec![1; n + 1];
    let mut pow3 = vec![1; n + 1];
    for i in 1..=n {
        pow2[i] = 2 * pow2[i - 1] % M;
        pow3[i] = 3 * pow3[i - 1] % M;
    }
    for i in 0..n {
        if l[i] == '1' {
            let d = n - i - 1;
            result = (result + pow2[ones] * pow3[d] % M) % M;
            ones += 1;
        }
    }
    result = (result + pow2[ones]) % M;
    println!("{}", result);
}
