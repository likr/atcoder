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

fn eval(x: &mut Vec<usize>, b: &Vec<Vec<usize>>, c: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut board = [[false; 3]; 3];
    for i in 0..9 {
        let xi = x[i];
        board[xi / 3][xi % 3] = if i % 2 == 0 { true } else { false };
    }
    // eprintln!("{:?}", board);
    let mut score = (0, 0);
    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == board[i + 1][j] {
                score.0 += b[i][j];
            } else {
                score.1 += b[i][j];
            }
        }
    }
    for i in 0..3 {
        for j in 0..2 {
            if board[i][j] == board[i][j + 1] {
                score.0 += c[i][j];
            } else {
                score.1 += c[i][j];
            }
        }
    }
    score
}

fn dfs(x: &mut Vec<usize>, d: usize, b: &Vec<Vec<usize>>, c: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut used = [false; 9];
    for i in 0..d {
        used[x[i]] = true;
    }
    return if d == 8 {
        x[8] = (0..9).filter(|&i| !used[i]).nth(0).unwrap();
        eval(x, b, c)
    } else {
        let mut scores = vec![];
        for i in (0..9).filter(|&i| !used[i]) {
            x[d] = i;
            scores.push(dfs(x, d + 1, b, c));
        }
        if d % 2 == 0 {
            *scores.iter().max_by_key(|&(s, _)| s).unwrap()
        } else {
            *scores.iter().max_by_key(|&(_, s)| s).unwrap()
        }
    };
}

fn main() {
    input! {
        b: [[usize; 3]; 2],
        c: [[usize; 2]; 3],
    }
    // let mut x = vec![3, 0, 4, 2, 1, 5, 6, 7, 8];
    // eprintln!("{:?}", eval(&mut x, &b, &c));
    let mut x = vec![0; 9];
    let (s1, s2) = dfs(&mut x, 0, &b, &c);
    println!("{}", s1);
    println!("{}", s2);
}
