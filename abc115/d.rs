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

fn solve(level: usize, x: usize, layers: &Vec<usize>, ps: &Vec<usize>) -> usize {
    if x == 0 {
        return 0;
    }
    if x >= layers[level] {
        return ps[level];
    }
    if level == 0 {
        return 1;
    }
    let mut s = 0;
    s += solve(level - 1, x - 1, layers, ps);
    if x >= layers[level] / 2 + 1 {
        s += 1;
        s += solve(level - 1, x - layers[level] / 2 - 1, layers, ps);
    }
    s
}

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut layers = vec![0; n + 1];
    layers[0] = 1;
    for i in 1..n + 1 {
        layers[i] = 2 * layers[i - 1] + 3;
    }
    let mut ps = vec![0; n + 1];
    ps[0] = 1;
    for i in 1..n + 1 {
        ps[i] = 2 * ps[i - 1] + 1;
    }
    println!("{}", solve(n, x, &layers, &ps));
}
