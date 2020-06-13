use ordered_float::*;
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
        m: usize,
        xyc: [(f64, f64, usize); n + m],
    }
    let k = n + m;
    let mut result = INF as f64;
    for x in 0..2usize.pow(m as u32) {
        let mut nodes = (0..n).collect::<HashSet<_>>();
        for i in 0..m {
            if 1 << i & x > 0 {
                nodes.insert(n + i);
            }
        }
        let mut edges = vec![];
        for i in 0..k {
            let (xi, yi, ci) = xyc[i];
            for j in 0..i {
                if nodes.contains(&i) && nodes.contains(&j) {
                    let (xj, yj, cj) = xyc[j];
                    let dx = xi - xj;
                    let dy = yi - yj;
                    let d = (dx * dx + dy * dy).sqrt();
                    let d = if ci == cj { d } else { 10. * d };
                    edges.push((i, j, d));
                }
            }
        }
        edges.sort_by_key(|&(_, _, d)| OrderedFloat::from(d));

        let mut components = UnionFind::new(k);
        let mut s = 0.;
        for &(i, j, d) in &edges {
            if components.union(i, j) {
                s += d;
            }
        }
        if s < result {
            result = s;
        }
    }
    println!("{}", result);
}
