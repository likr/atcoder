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
        k: usize,
        a: [Usize1; n],
    }

    if k <= n {
        let mut u = 0;
        for _ in 0..k {
            u = a[u];
        }
        println!("{}", u + 1);
        return;
    }

    let mut distance = vec![INF; n];
    distance[0] = 0usize;
    let mut u = 0;
    let mut l = 1usize;
    loop {
        if distance[a[u]] != INF {
            break;
        }
        distance[a[u]] = distance[u] + 1;
        u = a[u];
        l += 1;
    }
    // eprintln!("{:?}", distance);
    let cycle = l - distance[a[u]];
    let m = (k - distance[a[u]]) % cycle + distance[a[u]];
    // eprintln!("{} {} {}", distance[a[u]], l, m);

    let mut u = 0;
    for _ in 0..m {
        u = a[u];
    }
    println!("{}", u + 1);
}
