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
    }
    let mut map = vec![vec![false; n + 2]; n + 2];
    let mut result = vec![vec!['.'; n + 2]; n + 2];
    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            if (i % 5 == 1 && j % 5 == 2)
                || (i % 5 == 2 && j % 5 == 4)
                || (i % 5 == 3 && j % 5 == 1)
                || (i % 5 == 4 && j % 5 == 3)
                || (i % 5 == 0 && j % 5 == 0)
            {
                count += 1;
                result[i][j] = 'X';
                map[i][j] = true;
                map[i - 1][j] = true;
                map[i][j - 1] = true;
                map[i + 1][j] = true;
                map[i][j + 1] = true;
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            if !map[i][j] {
                count += 1;
                result[i][j] = 'X';
                map[i][j] = true;
            }
        }
    }
    // for i in 1..=n {
    //     for j in 1..=n {
    //         eprint!("{}", if map[i][j] { 'X' } else { '.' });
    //     }
    //     eprintln!("");
    // }
    eprintln!("{}", count);
    for i in 1..=n {
        for j in 1..=n {
            print!("{}", result[i][j]);
        }
        println!("");
    }
}
