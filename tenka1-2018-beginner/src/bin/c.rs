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
      n: usize,
      mut a: [usize; n],
    }
    a.sort();
    let mut q = VecDeque::new();
    for &ai in &a {
        q.push_back(ai);
    }
    let mut p = VecDeque::new();
    p.push_back(q.pop_front().unwrap());
    while !q.is_empty() {
        // println!("{:?} {:?}", p, q);
        let qf = q.front().unwrap();
        let qb = q.back().unwrap();
        let pf = p.front().unwrap();
        let pb = p.back().unwrap();
        let d1 = max(qf, pf) - min(qf, pf);
        let d2 = max(qf, pb) - min(qf, pb);
        let d3 = max(qb, pf) - min(qb, pf);
        let d4 = max(qb, pb) - min(qb, pb);
        if d1 >= d2 && d1 >= d3 && d1 >= d4 {
            let v = q.pop_front().unwrap();
            p.push_front(v);
        } else if d2 >= d1 && d2 >= d3 && d2 >= d4 {
            let v = q.pop_front().unwrap();
            p.push_back(v);
        } else if d3 >= d1 && d3 >= d2 && d3 >= d4 {
            let v = q.pop_back().unwrap();
            p.push_front(v);
        } else if d4 >= d1 && d4 >= d2 && d4 >= d3 {
            let v = q.pop_back().unwrap();
            p.push_back(v);
        }
    }
    let p = p.iter().collect::<Vec<_>>();
    // println!("{:?}", p);
    let mut s = 0;
    for i in 1..n {
        s += max(p[i], p[i - 1]) - min(p[i], p[i - 1])
    }
    println!("{}", s);
}
