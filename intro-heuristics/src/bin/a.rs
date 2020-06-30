use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::time::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn ss(n: usize) -> isize {
    (n * (n + 1) / 2) as isize
}

fn calc_score(t: &Vec<usize>, c: &Vec<isize>, s: &Vec<Vec<isize>>) -> isize {
    let n = c.len();
    let d = t.len();
    let mut last = vec![0; n];
    let mut score = 0;
    for i in 0..d {
        let ti = t[i];
        score += s[i][ti];
        last[ti] = i + 1;
        for j in 0..n {
            score -= c[j] * (i + 1 - last[j]) as isize;
        }
    }
    score
}

fn initial_guess(c: &Vec<isize>, s: &Vec<Vec<isize>>) -> Vec<usize> {
    let n = c.len();
    let d = s.len();
    let mut t = vec![0; d];
    let mut last = vec![0; n];
    for i in 0..d {
        t[i] = (0..n)
            .max_by_key(|&j| {
                s[i][j]
                    - (0..n)
                        .map(|k| c[k] * (i + 1 - last[k]) as isize)
                        .sum::<isize>()
            })
            .unwrap();
        last[t[i]] = i + 1;
    }
    t
}

fn improve(
    t: &mut Vec<usize>,
    score: isize,
    c: &Vec<isize>,
    s: &Vec<Vec<isize>>,
    di: usize,
) -> isize {
    let n = c.len();
    let d = s.len();

    let mut last = vec![0; n];
    for i in 0..di {
        last[t[i]] = i + 1;
    }
    let mut next = vec![d + 1; n];
    for i in (di + 1..d).rev() {
        next[t[i]] = i + 1;
    }

    let f =
        |j| s[di][j] - c[j] * (ss(di - last[j]) + ss(next[j] - di - 2) - ss(next[j] - last[j] - 1));
    let remove_score = f(t[di]);

    t[di] = (0..n).max_by_key(|&j| f(j)).unwrap();
    score + f(t[di]) - remove_score
}

fn main() {
    let start = SystemTime::now();
    let limit = Duration::from_millis(1950);
    let n = 26;
    input! {
        d: usize,
        c: [isize; n],
        s: [[isize; n]; d],
    }
    let mut t = initial_guess(&c, &s);
    let mut rng = thread_rng();
    let mut score = calc_score(&t, &c, &s);
    let mut best_score = score;
    let mut best_t = t.clone();
    eprintln!("{}", score);
    let mut iter = 0;
    loop {
        let prev_score = score;
        for di in 0..d {
            score = improve(&mut t, score, &c, &s, di);
        }
        if prev_score == score {
            if score > best_score {
                eprintln!("{}", score);
                best_score = score;
                best_t = t.clone()
            } else {
                t = best_t.clone();
            }
            let d1 = rng.gen_range(0, d);
            let d2 = rng.gen_range(0, d);
            t.swap(d1, d2);
            score = calc_score(&t, &c, &s);
        }
        iter += 1;
        if start.elapsed().unwrap() >= limit {
            break;
        }
    }
    eprintln!("{}", iter);
    eprintln!("{}", best_score);
    for &ti in &best_t {
        println!("{}", ti + 1);
    }
}
