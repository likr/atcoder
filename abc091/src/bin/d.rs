use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut result = 0;
    for k in 0..30 {
        let t = 2usize.pow(k as u32);
        let mut c = b.iter().map(|&bi| bi % (2 * t)).collect::<Vec<usize>>();
        c.sort();
        let mut count = 0usize;
        for i in 0..n {
            let ai = a[i] % (2 * t);
            let x = c.lower_bound_by_key(&t, |&ci| ai + ci);
            let y = c.lower_bound_by_key(&(2 * t), |&ci| ai + ci);
            let z = c.lower_bound_by_key(&(3 * t), |&ci| ai + ci);
            count += n - z + y - x;
        }
        dbg!(count);
        if count % 2 == 1 {
            result |= 1 << k;
        }
    }
    println!("{}", result);
}
