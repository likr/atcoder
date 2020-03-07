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
        p: usize,
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .rev()
        .collect::<Vec<_>>();

    if p == 5 || p == 2 {
        let mut count = 0;
        for i in 0..n {
            if s[i] % p == 0 {
                count += n - i;
            }
        }
        println!("{}", count);
        return;
    }

    let mut pow10 = vec![1usize; n];
    for i in 1..n {
        pow10[i] = (pow10[i - 1] * 10) % p;
    }
    // eprintln!("{:?}", pow10);

    let t = s
        .iter()
        .enumerate()
        .map(|(i, &d)| (d * pow10[i]) % p)
        .collect::<Vec<_>>();
    // eprintln!("{:?}", t);

    let mut acc = vec![0usize; n];
    acc[0] = t[0];
    for i in 1..n {
        acc[i] = (acc[i - 1] + t[i]) % p;
    }
    // eprintln!("{:?}", acc);

    let mut count = vec![0usize; p];
    for i in 0..n {
        count[acc[i]] += 1;
    }

    let mut result = count[0] * (count[0] + 1) / 2;
    for i in 1..p {
        if count[i] > 0 {
            result += (count[i] - 1) * count[i] / 2;
        }
    }
    println!("{}", result);
}
