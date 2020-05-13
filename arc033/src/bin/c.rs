#![feature(map_first_last)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use segment_tree::ops::Add;
use segment_tree::SegmentPoint;
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
        tx: [(usize, usize); q],
    }
    let n = 2000001;
    let mut tree = SegmentPoint::build(vec![0; n], Add);
    for &(ti, xi) in &tx {
        if ti == 1 {
            tree.modify(xi, 1);
        } else {
            let mut l = 0;
            let mut r = n;
            while r - l > 1 {
                let m = (l + r) / 2;
                if tree.query(0, m) < xi {
                    l = m;
                } else {
                    r = m;
                }
            }
            println!("{}", l);
            tree.modify(l, 0);
        }
    }
}
