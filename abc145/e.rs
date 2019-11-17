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
        t: usize,
        ab: [(usize, usize); n],
    }
    let mut a = ab.iter().map(|&(ai, _)| ai).collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, bi)| bi).collect::<Vec<_>>();
    let mut dp1 = vec![vec![0; t]; n];
    for j in a[0]..t {
        dp1[0][j] = b[0];
    }
    for i in 1..n {
        for j in 0..a[i] {
            if j < t {
                dp1[i][j] = dp1[i - 1][j];
            }
        }
        for j in a[i]..t {
            if j < t {
                dp1[i][j] = std::cmp::max(dp1[i - 1][j], dp1[i - 1][j - a[i]] + b[i]);
            }
        }
    }

    a.reverse();
    b.reverse();
    let mut dp2 = vec![vec![0; t]; n];
    for j in a[0]..t {
        dp2[0][j] = b[0];
    }
    for i in 1..n {
        for j in 0..a[i] {
            if j < t {
                dp2[i][j] = dp2[i - 1][j];
            }
        }
        for j in a[i]..t {
            if j < t {
                dp2[i][j] = std::cmp::max(dp2[i - 1][j], dp2[i - 1][j - a[i]] + b[i]);
            }
        }
    }

    a.reverse();
    b.reverse();
    dp2.reverse();

    let mut result = std::cmp::max(
        dp1[n - 2][t - 1] + b[n - 1],
        dp2[1][t - 1] + b[0]);
    for i in 1..(n - 1) {
        for j in 0..t {
            let s = dp1[i - 1][j] + dp2[i + 1][t - 1 - j] + b[i];
            if s > result {
                result = s;
            }
        }
    }

    println!("{}", result);
}
