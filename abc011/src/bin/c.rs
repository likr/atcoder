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
        ng: [usize; 3],
    }
    let ng = ng.into_iter().collect::<HashSet<_>>();
    let mut count = 0;
    if ng.contains(&n) {
        n = 0;
        count = 101;
    }
    while n > 0 {
        if n <= 3 {
            n = 0;
        } else if !ng.contains(&(n - 3)) {
            n -= 3;
        } else if !ng.contains(&(n - 2)) {
            n -= 2;
        } else if !ng.contains(&(n - 1)) {
            n -= 1;
        } else {
            n = 0;
            count += 100;
        }
        count += 1;
    }
    if count <= 100 {
        println!("YES");
    } else {
        println!("NO");
    }
}
