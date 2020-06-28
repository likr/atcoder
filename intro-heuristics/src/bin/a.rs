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

fn improve_random<R: Rng>(
    t: &mut Vec<usize>,
    score: isize,
    c: &Vec<isize>,
    s: &Vec<Vec<isize>>,
    rng: &mut R,
) -> isize {
    let n = c.len();
    let d = s.len();
    let di = rng.gen_range(0, d);
    let qi = rng.gen_range(0, n);
    let old = t[di];
    t[di] = qi;
    let new_score = calc_score(&t, &c, &s);
    if new_score <= score {
        t[di] = old;
        score
    } else {
        new_score
    }
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
    let old = t[di];

    let mut last = vec![0; n];
    for i in 0..=di {
        last[t[i]] = i + 1;
    }
    let mut next = vec![d; n];
    for i in di + 1..d {
        next[t[i]] = i + 1;
    }

    t[di] = (0..n)
        .max_by_key(|&j| {
            s[di][j] + c[j] * (di + 1 - last[j]) as isize
                - s[di][old]
                - c[old] * (next[old] + 1 - di) as isize
        })
        .unwrap();

    let new_score = calc_score(&t, &c, &s);
    if new_score <= score {
        t[di] = old;
        score
    } else {
        new_score
    }
}

fn main() {
    let start = SystemTime::now();
    let limit1 = Duration::from_millis(1500);
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
    let mut iter = 0;
    eprintln!("{}", score);
    loop {
        if start.elapsed().unwrap() < limit1 && rng.gen_ratio(1, 2) {
            for di in 0..d {
                score = improve(&mut t, score, &c, &s, di);
            }
        } else {
            score = improve_random(&mut t, score, &c, &s, &mut rng);
        };
        iter += 1;
        if start.elapsed().unwrap() >= limit {
            break;
        }
    }
    eprintln!("{}", iter);
    eprintln!("{}", score);
    for &ti in &t {
        println!("{}", ti + 1);
    }
}
