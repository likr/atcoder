use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        n: usize,
        d: [[usize; n]; n],
        q: usize,
        p: [usize; q],
    }
    let mut acc = vec![vec![0; n + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            acc[i][j] = acc[i][j - 1] + acc[i - 1][j] - acc[i - 1][j - 1] + d[i - 1][j - 1];
        }
    }
    // println!("{:?}", acc);

    let mut cache = HashMap::new();
    let mut p_max = 0;
    for i in 0..q {
        if p[i] > p_max {
            p_max = p[i];
        }
    }
    let mut score = vec![0; p_max];
    for i in 0..p_max {
        let mut result = 0;
        let c = i + 1;
        for a in 1..c + 1 {
            if c % a != 0 {
                continue;
            }
            let b = c / a;
            if a > n || b > n {
                continue;
            }
            // println!("{} {}", a, b);
            let s = if cache.contains_key(&(a, b)) {
                cache[&(a, b)]
            } else {
                let mut r = 0;
                for i in 0..n - a + 1 {
                    for j in 0..n - b + 1 {
                        let s = acc[i + a][j + b] - acc[i][j + b] - acc[i + a][j] + acc[i][j];
                        if s > r {
                            r = s;
                        }
                    }
                }
                cache.insert((a, b), r);
                r
            };
            if s > result {
                result = s;
            }
        }
        score[i] = result;
    }

    for &pi in &p {
        let mut max = 0;
        for i in 0..pi {
            if score[i] > max {
                max = score[i];
            }
        }
        println!("{}", max);
    }
}
