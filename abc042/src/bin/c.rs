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
        k: usize,
        d: [usize; k],
    }
    let d = d.into_iter().collect::<HashSet<_>>();
    'outer: for y in n.. {
        let mut x = y;
        while x > 0 {
            if d.contains(&(x % 10)) {
                continue 'outer;
            }
            x /= 10;
        }
        println!("{}", y);
        return;
    }
}
