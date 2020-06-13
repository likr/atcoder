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
        s: Chars,
    }
    let n = s.len();
    let mut candidate = HashSet::new();
    for k in 1..=3 {
        for i in 0..n {
            if i + k > n {
                break;
            }
            for x in 0..2usize.pow(k as u32) {
                candidate.insert(
                    (0..k)
                        .map(|j| if 1 << j & x > 0 { s[i + j] } else { '.' })
                        .collect::<String>(),
                );
            }
        }
    }
    eprintln!("{:?}", candidate);
    println!("{}", candidate.len());
}
