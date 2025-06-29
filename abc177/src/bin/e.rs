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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn find_primes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            for j in (2 * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a_max = *a.iter().max().unwrap();
    let is_prime = find_primes(a_max);
    let primes = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &f)| if f { Some(i) } else { None })
        .collect::<Vec<_>>();
    debug!(primes);
    let mut count = HashMap::new();
    let mut ans1 = true;
    for &ai in a.iter() {
        let mut x = ai;
        for &p in primes.iter() {
            if p * p > x {
                break;
            }
            if x % p == 0 {
                while x % p == 0 {
                    x /= p;
                }
                if count.contains_key(&p) {
                    ans1 = false;
                }
                *count.entry(p).or_insert(0) += 1;
            }
        }
        if x > 1 {
            if count.contains_key(&x) {
                ans1 = false;
            }
            *count.entry(x).or_insert(0) += 1;
        }
    }
    if ans1 {
        println!("pairwise coprime");
        return;
    }
    debug!(count);
    for &p in primes.iter() {
        if let Some(&c) = count.get(&p) {
            if c == n {
                println!("not coprime");
                return;
            }
        }
    }
    println!("setwise coprime");
}
