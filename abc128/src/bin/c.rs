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
        s: [[Usize1]; m],
        p: [usize; m],
    }
    let mut result = 0usize;
    for x in 0..2usize.pow(n as u32) {
        let mut on = HashSet::new();
        for i in 0..n {
            if 1 << i & x > 0 {
                on.insert(i);
            }
        }
        let mut count = vec![0; m];
        for j in 0..m {
            for &sjk in &s[j] {
                if on.contains(&sjk) {
                    count[j] += 1;
                }
            }
        }
        if (0..m).all(|j| count[j] % 2 == p[j]) {
            result += 1;
        }
    }
    println!("{}", result);
}
