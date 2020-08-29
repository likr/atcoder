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

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        gcd(b, a)
    } else if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn is_pairwise_coprime(a: &Vec<usize>) -> bool {
    let primes = {
        let mut primes = vec![true; 1001];
        primes[0] = false;
        primes[1] = false;
        for i in 2..primes.len() {
            if primes[i] {
                for j in 2.. {
                    if i * j >= primes.len() {
                        break;
                    }
                    primes[i * j] = false;
                }
            }
        }
        primes
            .into_iter()
            .enumerate()
            .filter(|&(_, f)| f)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    };

    let n = a.len();
    let mut b = a.clone();
    let mut visited = HashSet::new();
    for i in 0..n {
        for &p in &primes {
            if b[i] % p == 0 {
                if visited.contains(&p) {
                    return false;
                } else {
                    visited.insert(p);
                }
            }
            while b[i] % p == 0 {
                b[i] /= p;
            }
            if b[i] == 1 {
                break;
            }
        }
        if b[i] > 1 {
            if visited.contains(&b[i]) {
                return false;
            } else {
                visited.insert(b[i]);
            }
        }
    }
    return true;
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if is_pairwise_coprime(&a) {
        println!("pairwise coprime");
        return;
    }

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }
    if g == 1 {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}
