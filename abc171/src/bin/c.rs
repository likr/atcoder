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
        mut n: usize,
    }
    let mut d = vec![];
    while n > 0 {
        n -= 1;
        d.push(n % 26);
        n /= 26;
    }
    d.reverse();
    eprintln!("{:?}", d);
    println!(
        "{}",
        d.iter()
            .map(|i| ('a' as usize + i) as u8 as char)
            .collect::<String>()
    );
}
