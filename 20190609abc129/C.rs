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

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

static X : usize = 1000000007;

fn fib(n: usize) -> usize {
    let mut s0 = 0;
    let mut s1 = 1;
    for _ in 0..n {
        let s = (s0 + s1) % X;
        s0 = s1;
        s1 = s;
    }
    s0
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut intervals = Vec::new();
    intervals.resize(m + 1, 0);

    if m == 0 {
        intervals[0] = n;
    } else {
        intervals[0] = a[0] - 1;
        for i in 1..m {
            if a[i] - a[i - 1] < 2 {
                println!("0");
                return;
            }
            intervals[i] = a[i] - a[i - 1] - 2;
        }
        intervals[m] = n - a[m - 1] - 1;
    }

    let mut result = 1;
    for v in intervals.iter() {
        result = ((result % X) * fib(v + 1)) % X;
    }

    println!("{}", result);
}
