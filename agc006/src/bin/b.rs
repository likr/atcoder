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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn check(x: &Vec<usize>) -> usize {
    if x.len() == 1 {
        return x[0];
    }
    let mut y = vec![];
    for i in 2..x.len() {
        let z = [x[i - 2], x[i - 1], x[i]];
        y.push(z.iter().sum::<usize>() - z.iter().min().unwrap() - z.iter().max().unwrap());
    }
    check(&y)
}

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    if n == 2 {
        if x == 2 {
            println!("Yes");
            println!("1");
            println!("2");
            println!("3");
        } else {
            println!("No");
        }
    } else if x == 1 || x == 2 * n - 1 {
        println!("No");
    } else {
        let mut sol = vec![0; 2 * n - 1];
        let mut nums = if x == 2 * n - 2 {
            sol[n - 3] = x - 1;
            sol[n - 2] = x + 1;
            sol[n - 1] = x;
            sol[n] = x - 2;
            sol[n + 1] = x - 3;
            (1..2 * n)
                .filter(|&y| y < x - 3 || x + 1 < y)
                .collect::<Vec<usize>>()
        } else if x == 2 {
            sol[n - 3] = x + 1;
            sol[n - 2] = x - 1;
            sol[n - 1] = x;
            sol[n] = x + 2;
            sol[n + 1] = x + 3;
            (1..2 * n)
                .filter(|&y| y < x - 1 || x + 3 < y)
                .collect::<Vec<usize>>()
        } else {
            sol[n - 3] = x + 1;
            sol[n - 2] = x - 1;
            sol[n - 1] = x;
            sol[n] = x + 2;
            sol[n + 1] = x - 2;
            (1..2 * n)
                .filter(|&y| y < x - 2 || x + 2 < y)
                .collect::<Vec<usize>>()
        };
        for i in 0..n - 3 {
            sol[i] = nums.pop().unwrap();
        }
        for i in n + 2..2 * n - 1 {
            sol[i] = nums.pop().unwrap();
        }
        debug!(check(&sol) == x);
        println!("Yes");
        for &s in &sol {
            println!("{}", s);
        }
    }
}
