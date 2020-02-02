use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;

const B1: usize = 1009;
const B2: usize = 1007;
const M1: usize = 1000000007;
const M2: usize = 1000000009;

fn main() {
    input! {
      n: usize,
      s: Chars,
    }
    let mut hash1 = vec![0; n + 1];
    let mut hash2 = vec![0; n + 1];
    let mut pow1 = vec![1; n + 1];
    let mut pow2 = vec![1; n + 1];
    for i in 0..n {
        hash1[i + 1] = (hash1[i] + s[i] as usize) * B1 % M1;
        hash2[i + 1] = (hash2[i] + s[i] as usize) * B2 % M2;
        pow1[i + 1] = pow1[i] * B1 % M1;
        pow2[i + 1] = pow2[i] * B2 % M2;
    }

    let mut hashes = HashMap::new();
    let mut result = 0;
    for k in 1..=n / 2 {
        let mut ok = false;
        for i in 0..=n - k {
            let h = (
                ((hash1[i + k] + M1 - hash1[i] * pow1[k] % M1) % M1 + M1) % M1,
                ((hash2[i + k] + M2 - hash2[i] * pow2[k] % M2) % M2 + M2) % M2,
            );
            if let Some(&j) = hashes.get(&h) {
                if i - j >= k {
                    ok = true;
                }
            } else {
                hashes.insert(h, i);
            }
        }
        // println!("{:?}", hashes);
        if !ok {
            break;
        }
        result = k;
    }
    println!("{}", result);
}
