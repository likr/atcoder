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
        q: usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                c: char,
                x: usize,
            }
            queue.push_back((c, x));
        } else {
            input! {
                mut d: usize,
            }
            let mut delete = HashMap::new();
            while d > 0 {
                if let Some((c, x)) = queue.pop_front() {
                    if x <= d {
                        *delete.entry(c).or_insert(0) += x;
                        d -= x;
                    } else {
                        *delete.entry(c).or_insert(0) += d;
                        queue.push_front((c, x - d));
                        d = 0;
                    }
                } else {
                    break;
                }
            }
            let mut out = 0usize;
            for &v in delete.values() {
                out += v * v;
            }
            println!("{}", out);
        }
    }
}
