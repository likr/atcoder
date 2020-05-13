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
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }
    let mut set = HashMap::new();
    for &di in &d {
        *set.entry(di).or_insert(0) += 1;
    }
    for &ti in &t {
        if !set.contains_key(&ti) || set[&ti] == 0 {
            println!("NO");
            return;
        }
        *set.get_mut(&ti).unwrap() -= 1;
    }
    println!("YES");
}
