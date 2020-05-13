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
        st: [(String, usize); n],
        x: String,
    }
    let k = st.iter().map(|(s, _)| s).position(|s| *s == x).unwrap();
    let mut result = 0;
    for i in k + 1..n {
        result += st[i].1;
    }
    println!("{}", result);
}
