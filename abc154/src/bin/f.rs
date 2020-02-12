use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

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

fn f(r: usize, c: usize, table: &Vec<(usize, usize)>) -> usize {
    (table[r + c].0 * table[r].1 % M) * table[c].1 % M
}

fn g(r: usize, c: usize, table: &Vec<(usize, usize)>) -> usize {
    let mut s = 0;
    for i in 0..=r {
        s = (s + f(i + 1, c, table)) % M;
    }
    s
}

fn main() {
    input! {
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
    }
    let mut table = vec![(1, 1); r2 + c2 + 2];
    for i in 1..table.len() {
        table[i].0 = i * table[i - 1].0 % M;
        table[i].1 = inv(table[i].0);
    }
    let mut result = g(r2, c2, &table);
    result = (result + M - g(r2, c1 - 1, &table)) % M;
    result = (result + M - g(r1 - 1, c2, &table)) % M;
    result = (result + g(r1 - 1, c1 - 1, &table)) % M;
    println!("{}", result);
}
