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
        mut xy: [(isize, isize); n],
    }
    xy.sort_by_key(|&(x, _)| x);
    let mut x = HashMap::new();
    let mut x_index = HashMap::new();
    for i in 0..n {
        x.insert(xy[i].0, i);
        x_index.insert(i + 1, xy[i].0);
    }
    xy.sort_by_key(|&(_, y)| y);
    let mut y = HashMap::new();
    let mut y_index = HashMap::new();
    for i in 0..n {
        y.insert(xy[i].1, i);
        y_index.insert(i + 1, xy[i].1);
    }

    let mut count = vec![vec![0; n]; n];
    for i in 0..n {
        let (xi, yi) = xy[i];
        count[x[&xi]][y[&yi]] += 1;
    }
    // eprintln!("{:?}", count);
    let mut count_acc = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            count_acc[i][j] = count[i - 1][j - 1] + count_acc[i - 1][j] + count_acc[i][j - 1]
                - count_acc[i - 1][j - 1];
        }
    }
    // eprintln!("{:?}", count_acc);
    let mut s = INF as isize;
    for i in 1..=n {
        for j in 1..=n {
            for k in 1..i {
                for l in 1..j {
                    if count_acc[i][j] - count_acc[k - 1][j] - count_acc[i][l - 1]
                        + count_acc[k - 1][l - 1]
                        >= m
                    {
                        // eprintln!(
                        //     "{} {} {} {}",
                        //     x_index[&i], x_index[&k], y_index[&j], y_index[&l]
                        // );
                        let t = (x_index[&i] - x_index[&k]) * (y_index[&j] - y_index[&l]);
                        s = min(s, t);
                    }
                }
            }
        }
    }
    println!("{}", s);
}
