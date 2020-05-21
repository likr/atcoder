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
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).ok();
    let c = c.split(' ').filter(|&s| !s.is_empty()).collect::<Vec<_>>();
    print!("{}", c.join(","));
}
