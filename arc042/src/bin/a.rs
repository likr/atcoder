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
        m: usize,
        mut a: [Usize1; m],
    }
    a.reverse();
    let mut visited = vec![false; n];
    for &ai in &a {
        if !visited[ai] {
            println!("{}", ai + 1);
            visited[ai] = true;
        }
    }
    for i in 0..n {
        if !visited[i] {
            println!("{}", i + 1);
            visited[i] = true;
        }
    }
}
