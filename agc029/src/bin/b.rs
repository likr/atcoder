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
        a: [usize; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }
    let mut keys = count.keys().map(|&k| k).collect::<Vec<usize>>();
    keys.sort();
    keys.reverse();

    let mut b = vec![];
    for i in 1..40 {
        b.push(2usize.pow(i));
    }
    b.reverse();

    let mut result = 0;
    for &bi in &b {
        for &k1 in &keys {
            if k1 >= bi {
                continue;
            }
            let c1 = count[&k1];
            let k2 = bi - k1;
            if let Some(&c2) = count.get(&k2) {
                if k1 == k2 {
                    let c = c1 / 2;
                    // eprintln!("{} {} {}", k1, k2, c);
                    result += c;
                    *count.get_mut(&k1).unwrap() -= c;
                } else {
                    let c = min(c1, c2);
                    // eprintln!("{} {} {}", k1, k2, c);
                    result += c;
                    *count.get_mut(&k1).unwrap() -= c;
                    *count.get_mut(&k2).unwrap() -= c;
                }
            }
        }
    }
    println!("{}", result);
}
