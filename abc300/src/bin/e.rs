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
const M: usize = 998244353;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn main() {
    input! {
        n: usize,
    }
    let mut p = HashMap::new();
    p.insert(1, 1);
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(1));
    while let Some(Reverse(x)) = queue.pop() {
        if visited.contains(&x) {
            continue;
        }
        visited.insert(x);
        for i in 2..=6 {
            if x * i <= n {
                *p.entry(x * i).or_insert(0) += p[&x] * inv(5) % M;
                *p.entry(x * i).or_insert(0) %= M;
                queue.push(Reverse(x * i))
            }
        }
    }
    if let Some(&result) = p.get(&n) {
        println!("{}", result);
    } else {
        println!("0");
    }
}
