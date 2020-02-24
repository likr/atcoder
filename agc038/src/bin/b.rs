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
        p: [usize; n],
    }
    let mut sorted_count = vec![0; n];
    for i in 1..n {
        if p[i - 1] < p[i] {
            sorted_count[i] = sorted_count[i - 1] + 1;
        }
    }

    let m = n - k + 1;
    let mut sorted = 0;
    for i in 0..m {
        if sorted_count[i + k - 1] >= 3 {
            sorted += 1;
        }
    }
    eprintln!("{} {:?}", sorted, sorted_count);

    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(p[i]);
    }
    let mut min_k = vec![0; m];
    let mut max_k = vec![0; m];
    min_k[0] = *set.iter().nth(0).unwrap();
    max_k[0] = *set.iter().last().unwrap();
    for i in 1..m {
        set.remove(&p[i - 1]);
        set.insert(p[i + k - 1]);
        min_k[i] = *set.iter().nth(0).unwrap();
        max_k[i] = *set.iter().last().unwrap();
    }
    // eprintln!("{:?}", min_k);
    // eprintln!("{:?}", max_k);
    let mut result = 1;
    for i in 1..m {
        if p[i - 1] > min_k[i] || max_k[i] > p[i + k - 1] {
            result += 1;
        }
    }
    if sorted > 0 {
        result -= sorted - 1;
    }
    println!("{}", result);
}
