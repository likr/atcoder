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
        a: Chars,
    }
    let n = a.len();
    let mut palindrome = vec![];
    for i in 0..n {
        let l = min(i, n - 1 - i);
        let mut j = 0;
        while j <= l && a[i - j] == a[i + j] {
            j += 1;
        }
        if j > 1 {
            palindrome.push((i + 1 - j, i + j - 1));
        }
    }
    for i in 1..n {
        let l = min(i, n - i);
        let mut j = 0;
        while j < l && a[i - j - 1] == a[i + j] {
            j += 1;
        }
        if j > 0 {
            palindrome.push((i - j, i + j - 1));
        }
    }
    eprintln!("{:?}", palindrome);

    let mut result = n * (n - 1) / 2 + 1;
    for &(i, j) in &palindrome {
        result -= (j - i + 1) / 2;
    }
    println!("{}", result);
}
