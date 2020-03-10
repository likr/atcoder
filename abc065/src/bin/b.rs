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
        a: [Usize1; n],
    }
    let mut i = 0;
    let mut c = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&a[i]) {
        visited.insert(i);
        i = a[i];
        c += 1;
        if i == 1 {
            println!("{}", c);
            return;
        }
    }
    println!("-1");
}
