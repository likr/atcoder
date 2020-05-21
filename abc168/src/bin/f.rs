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
        m: usize,
        mut abc: [(isize, isize, isize); n],
        mut def: [(isize, isize, isize); m],
    }
    abc.sort_by_key(|&(_, _, ci)| ci);
    def.sort_by_key(|&(di, _, _)| di);
    println!("{}", n);
}
