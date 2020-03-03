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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }
    let mut w = HashSet::new();
    for i in 0.. {
        if 100 * a * i > f {
            break;
        }
        for j in 0.. {
            let wab = 100 * (a * i + b * j);
            if wab > f {
                break;
            }
            if wab > 0 {
                w.insert(wab);
            }
        }
    }
    let mut s = HashSet::new();
    for i in 0.. {
        if c * i > f {
            break;
        }
        for j in 0.. {
            let scd = c * i + d * j;
            if scd > f {
                break;
            }
            s.insert(scd);
        }
    }

    // eprintln!("{:?}", w);
    // eprintln!("{:?}", s);
    let mut result = (*w.iter().nth(0).unwrap(), 0);
    for &wi in w.iter() {
        for &sj in s.iter() {
            if wi + sj <= f && 100 * sj <= wi * e {
                // eprintln!("{} {}", wi, sj);
                if sj as f64 / (wi + sj) as f64 > result.1 as f64 / result.0 as f64 {
                    result = (wi + sj, sj);
                }
            }
        }
    }
    println!("{} {}", result.0, result.1);
}
