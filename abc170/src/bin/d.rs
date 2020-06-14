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

fn factors(n: usize) -> HashSet<usize> {
    let mut factors = HashSet::new();
    for d in 1.. {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            factors.insert(d);
            factors.insert(n / d);
        }
    }
    factors
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }
    a.sort();
    let mut f = HashSet::new();
    let mut result = 0;
    for i in 0..n {
        if count[&a[i]] == 1 && factors(a[i]).iter().all(|&x| !f.contains(&x)) {
            result += 1;
        }
        f.insert(a[i]);
    }
    println!("{}", result);
}
