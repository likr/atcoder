use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use union_find::{QuickFindUf, UnionBySize, UnionFind};

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    let n = 10;
    input! {
        mut a: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 'x' {
                a[i][j] = 'o';

                let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n * n);
                for x in 1..n {
                    for y in 0..n {
                        if a[x - 1][y] == a[x][y] {
                            components.union((x - 1) * n + y, x * n + y);
                        }
                    }
                }
                for x in 0..n {
                    for y in 1..n {
                        if a[x][y - 1] == a[x][y] {
                            components.union(x * n + y - 1, x * n + y);
                        }
                    }
                }

                let mut nc = 0;
                for x in 0..n {
                    for y in 0..n {
                        if a[x][y] == 'o' && components.find(x * n + y) == x * n + y {
                            nc += 1;
                        }
                    }
                }

                if nc == 1 {
                    println!("YES");
                    return;
                }

                a[i][j] = 'x';
            }
        }
    }

    println!("NO");
}
