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
        _n: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    }
    if c > d {
        if !(b..=d).any(|i| s[i - 1] == '.' && s[i] == '.' && s[i + 1] == '.') {
            eprintln!("1");
            println!("No");
            return;
        }
    }
    if (a + 1..=c).any(|i| s[i - 1] == '#' && s[i] == '#') {
        eprintln!("2");
        println!("No");
        return;
    }
    if (b + 1..=d).any(|i| s[i - 1] == '#' && s[i] == '#') {
        eprintln!("3");
        println!("No");
        return;
    }
    println!("Yes");
}
