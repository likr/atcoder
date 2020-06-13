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

fn mul(a: &[[usize; 3]; 3], b: &[[usize; 3]; 3], m: usize) -> [[usize; 3]; 3] {
    let mut c = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j] % m) % m;
            }
        }
    }
    c
}

fn pow(a: &[[usize; 3]; 3], mut p: usize, m: usize) -> [[usize; 3]; 3] {
    let mut c = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let mut d = a.clone();
    while p > 0 {
        if p % 2 == 1 {
            c = mul(&c, &d, m);
        }
        d = mul(&d, &d, m);
        p /= 2;
    }
    c
}

fn main() {
    input! {
        l: usize,
        a: usize,
        b: usize,
        m: usize,
    }
    let s_max = a + b * (l - 1);
    let n = (s_max as f64).log10() as usize + 1;
    let mut d = vec![0; n];
    d[0] = if a <= 9 {
        (min(s_max, 9) - a) / b + 1
    } else {
        0
    };
    let mut f = d[0];
    for i in 1..n {
        let e = 10usize.pow(i as u32 + 1) - 1;
        d[i] = if a <= e {
            (min(s_max, e) - a) / b + 1 - f
        } else {
            0
        };
        f += d[i];
    }
    for i in 0..n {
        if d[i] > 0 {
            d[i] -= 1;
            break;
        }
    }

    let mut s = (a % m, (a + b) % m);
    for i in 0..n {
        if d[i] == 0 {
            continue;
        }
        let e = 10usize.pow(i as u32 + 1) % m;
        let x = pow(&[[e, 0, 0], [1, 1, 0], [0, b, 1]], d[i], m);
        s = (
            (x[0][0] * s.0 % m + x[1][0] * s.1 % m + x[2][0]) % m,
            (x[0][1] * s.0 % m + x[1][1] * s.1 % m + x[2][1]) % m,
        );
    }
    println!("{:?}", s.0);
}
