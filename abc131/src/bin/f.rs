use petgraph::unionfind::*;
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
        xy: [(usize, usize); n],
    }

    let mut x_points = HashMap::new();
    let mut y_points = HashMap::new();
    for i in 0..n {
        let (xi, yi) = xy[i];
        x_points.entry(xi).or_insert(vec![]).push(i);
        y_points.entry(yi).or_insert(vec![]).push(i);
    }
    // eprintln!("{:?}", x_points);
    // eprintln!("{:?}", y_points);

    let mut components = UnionFind::new(n);
    for i in 0..n {
        let (xi, yi) = xy[i];
        if x_points[&xi].len() > 1 && y_points[&yi].len() > 1 {
            // eprintln!("{} {}", xi, yi);
            for &j in &x_points[&xi] {
                components.union(i, j);
            }
            for &j in &y_points[&yi] {
                components.union(i, j);
            }
        }
    }

    let mut points = vec![vec![]; n];
    for i in 0..n {
        points[components.find(i)].push(i);
    }
    let mut result = 0usize;
    for i in 0..n {
        if points[i].len() > 2 {
            let mut xs = HashSet::new();
            let mut ys = HashSet::new();
            for &i in &points[i] {
                let (xi, yi) = xy[i];
                xs.insert(xi);
                ys.insert(yi);
            }
            result += xs.len() * ys.len() - points[i].len();
        }
    }
    println!("{}", result);
}
