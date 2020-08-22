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
        l: [usize; n],
    }
    let mut count = 0;
    for k in 0..n {
        for j in 0..k {
            for i in 0..j {
                let mut edges = vec![l[i], l[j], l[k]];
                edges.sort();
                edges.dedup();
                if edges.len() == 3 && edges[0] + edges[1] > edges[2] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
