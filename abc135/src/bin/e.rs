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
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        k: i64,
        x0: i64,
        y0: i64,
    }
    let x = max(x0.abs(), y0.abs());
    let y = min(x0.abs(), y0.abs());
    if (x + y) % 2 == 1 && k % 2 == 0 {
        println!("-1");
    } else {
        let mut points = vec![];
        if x + y == k {
            points.push((x, y));
        } else if x + y < k && (x + y) % 2 == 1 {
            points.push((x, x - k));
            points.push(((3 * x - y + k) / 2, (x + y - k) / 2));
            points.push((x, y));
        } else {
            let c = (k - ((x + y) % k)) % k;
            let n = if x + y < k && (x + y) % 2 == 0 {
                2
            } else if c % 2 == 0 {
                (x + y + k - 1) / k
            } else {
                (x + y + k - 1) / k + 1
            };
            eprintln!("{} {}", c, n);
            let mut p = (0, 0);
            while p.1 - k > (x + y - k * n) / 2 {
                p.1 -= k;
                points.push(p.clone());
            }
            p.0 = k + (x + y - k * n) / 2;
            p.1 = (x + y - k * n) / 2;
            points.push(p.clone());
            while p.0 + k < x {
                p.0 += k;
                points.push(p.clone());
            }
            p.1 += k - (x - p.0);
            p.0 = x;
            points.push(p.clone());
            while p.1 + k < y {
                p.1 += k;
                points.push(p.clone());
            }
            if points.len() < n as usize {
                points.push((x, y));
            }
        }
        println!("{}", points.len());
        for (xi, yi) in points {
            let (xi, yi) = if x0.abs() < y0.abs() {
                (yi, xi)
            } else {
                (xi, yi)
            };
            println!(
                "{} {}",
                if x0 < 0 { -xi } else { xi },
                if y0 < 0 { -yi } else { yi }
            );
        }
    }
}
