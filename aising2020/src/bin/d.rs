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

fn rec(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&f) = cache.get(&n) {
        f
    } else {
        let mut m = n;
        let mut one_count = 0;
        while m > 0 {
            if m % 2 == 1 {
                one_count += 1;
            }
            m /= 2;
        }
        let f = rec(n % one_count, cache);
        cache.insert(n, f + 1);
        cache[&n]
    }
}

fn main() {
    input! {
        n: usize,
        mut x: Chars,
    }
    x.reverse();
    let one_count = x.iter().filter(|&&c| c == '1').count();

    let mut s = vec![1; n];
    let mut t = vec![1; n];
    for i in 1..n {
        s[i] = s[i - 1] * 2 % (one_count + 1);
        if one_count > 1 {
            t[i] = t[i - 1] * 2 % (one_count - 1);
        }
    }

    let mut sum_p = 0;
    let mut sum_m = 0;
    for i in 0..n {
        if x[i] == '1' {
            sum_p = (sum_p + s[i]) % (one_count + 1);
            if one_count > 1 {
                sum_m = (sum_m + t[i]) % (one_count - 1);
            }
        }
    }

    let mut result = vec![];
    let mut cache = HashMap::new();
    cache.insert(0, 0);
    for i in 0..n {
        let f = if x[i] == '0' {
            let xi = (sum_p + s[i]) % (one_count + 1);
            rec(xi, &mut cache) + 1
        } else {
            if one_count > 1 {
                let xi = (sum_m + one_count - 1 - t[i]) % (one_count - 1);
                rec(xi, &mut cache) + 1
            } else {
                0
            }
        };
        result.push(f);
    }
    result.reverse();
    for &f in &result {
        println!("{}", f);
    }
}
