use std::io::Read;

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
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut a = vec![0; m];
    let mut x = vec![0; m];
    for i in 0..m {
        a[i] = iter.next().unwrap().parse::<usize>().unwrap();
        let bi = iter.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..bi {
            let cij = iter.next().unwrap().parse::<usize>().unwrap();
            x[i] |= 1 << (cij - 1);
        }
    }
    // println!("{:?}", x);

    let inf = 10000000000;
    let max = 2usize.pow(n as u32);
    let mut dp = vec![inf; max];
    dp[0] = 0;

    for i in 0..m {
        for j in 0..max {
            let y = x[i] | j;
            let s = dp[j] + a[i];
            if s < dp[y] {
                dp[y] = s;
            }
        }
    }
    if dp[max - 1] == inf {
        println!("-1");
    } else {
        println!("{}", dp[max - 1]);
    }
}
