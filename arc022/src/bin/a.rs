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
        s: Chars,
    }
    let n = s.len();
    let mut i = 0;
    while i < n && s[i] != 'I' && s[i] != 'i' {
        i += 1;
    }
    if i == n {
        println!("NO");
        return;
    }
    while i < n && s[i] != 'C' && s[i] != 'c' {
        i += 1;
    }
    if i == n {
        println!("NO");
        return;
    }
    while i < n && s[i] != 'T' && s[i] != 't' {
        i += 1;
    }
    if i == n {
        println!("NO");
        return;
    }
    println!("YES");
}
