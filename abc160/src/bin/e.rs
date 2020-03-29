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
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut p: [usize; a],
        mut q: [usize; b],
        mut r: [usize; c],
    }
    p.sort();
    p.reverse();
    q.sort();
    q.reverse();
    r.sort();
    r.reverse();
    p.push(0);
    q.push(0);
    r.push(0);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut d = 0;
    let mut e = 0;
    let mut result = 0;
    for _ in 0..x + y {
        if d < x && ((p[i] >= q[j] && p[i] >= r[k]) || (e == y && p[i] >= r[k])) {
            // eprintln!("p");
            result += p[i];
            d += 1;
            i += 1;
        } else if e < y && ((q[j] >= p[i] && q[j] >= r[k]) || (d == x && q[j] >= r[k])) {
            // eprintln!("q");
            result += q[j];
            e += 1;
            j += 1;
        } else {
            // eprintln!("r");
            result += r[k];
            k += 1;
        }
    }
    println!("{}", result);
}
