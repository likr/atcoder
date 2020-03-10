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
        k: usize,
    }
    let mut s = s.into_iter().collect::<VecDeque<_>>();
    for _ in 0..k {
        let c = s.pop_front().unwrap();
        s.push_back(c);
    }
    println!("{}", s.into_iter().collect::<String>());
}
