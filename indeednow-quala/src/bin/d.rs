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
        h: usize,
        w: usize,
        c: [usize; w * h],
    }
    let n = h * w;
    let mut goal = (1..n).collect::<Vec<_>>();
    goal.push(0);
    if c == goal {
        println!("0");
        return;
    }

    let mut from_goal = HashMap::new();
    from_goal.insert(goal.clone(), 0);
    let mut queue = VecDeque::new();
    queue.push_back(goal);
    while let Some(x) = queue.pop_front() {
        let d = from_goal[&x];
        let k = (0..n).find(|&i| x[i] == 0).unwrap();
        let (i, j) = (k / w, k % w);
        let mut y = vec![];
        if i > 0 {
            y.push((i - 1, j));
        }
        if i + 1 < h {
            y.push((i + 1, j));
        }
        if j > 0 {
            y.push((i, j - 1));
        }
        if j + 1 < w {
            y.push((i, j + 1));
        }
        for &(i2, j2) in &y {
            let k2 = i2 * w + j2;
            let mut x2 = x.clone();
            x2.swap(k, k2);
            if !from_goal.contains_key(&x2) && d + 1 <= 12 {
                from_goal.insert(x2.clone(), d + 1);
                queue.push_back(x2);
            }
        }
    }
    if let Some(&c) = from_goal.get(&c) {
        println!("{}", c);
        return;
    }

    let mut from_start = HashMap::new();
    from_start.insert(c.clone(), 0);
    let mut queue = VecDeque::new();
    queue.push_back(c);
    while let Some(x) = queue.pop_front() {
        let k = (0..n).find(|&i| x[i] == 0).unwrap();
        let (i, j) = (k / w, k % w);
        let mut y = vec![];
        if i > 0 {
            y.push((i - 1, j));
        }
        if i + 1 < h {
            y.push((i + 1, j));
        }
        if j > 0 {
            y.push((i, j - 1));
        }
        if j + 1 < w {
            y.push((i, j + 1));
        }
        let d = from_start[&x];
        for &(i2, j2) in &y {
            let k2 = i2 * w + j2;
            let mut x2 = x.clone();
            x2.swap(k, k2);
            if let Some(&c) = from_goal.get(&x2) {
                println!("{}", d + c + 1);
                return;
            }
            if !from_start.contains_key(&x2) {
                from_start.insert(x2.clone(), d + 1);
                queue.push_back(x2);
            }
        }
    }
}
