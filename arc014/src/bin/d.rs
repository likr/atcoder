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
        all: isize,
        n: usize,
        m: usize,
        l: [isize; n],
        xy: [(isize, isize); m],
    }
    let mut xy = xy
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, (isize, isize))>>();
    xy.sort_by_key(|(_, (x, y))| x + y);

    for &(i, (x, y)) in &xy {
        let mut s = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && l[j - 1] + y >= l[j] - x {
                j += 1;
            }
            // eprintln!("{} {}", i, j);
            // eprintln!(" {} {}", l[i], l[j - 1]);
            s += min(all, l[j - 1] + y) - max(1, l[i] - x) + 1;
            i = j;
        }
        println!("{}", s);
    }
}
