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
        xy: [(f64, f64); n],
    }
    let x = xy.iter().map(|&(xi, _)| xi).collect::<Vec<_>>();
    let y = xy.iter().map(|&(_, yi)| yi).collect::<Vec<_>>();
    let mut result = 1000.;
    for i in 0..n {
        for j in 0..i {
            let dx = x[i] - x[j];
            let dy = y[i] - y[j];
            let r = (dx * dx + dy * dy).sqrt() / 2.;
            let cx = (x[i] + x[j]) / 2.;
            let cy = (y[i] + y[j]) / 2.;
            if r < result && (0..n).all(|l| (x[l] - cx) * (x[l] - cx) + (y[l] - cy) * (y[l] - cy) - r * r <= 1e-6) {
                result = r;
            }
        }
    }
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                let a = x[i] - x[j];
                let b = y[i] - y[j];
                let c = x[j] - x[k];
                let d = y[j] - y[k];
                let f = 2. * (a * d - b * c);
                let ni = x[i] * x[i] + y[i] * y[i];
                let nj = x[j] * x[j] + y[j] * y[j];
                let nk = x[k] * x[k] + y[k] * y[k];
                let cx = (d * (ni - nj) + b * (nk - nj)) / f;
                let cy = (c * (nj - ni) + a * (nj - nk)) / f;
                let r = ((x[i] - cx) * (x[i] - cx) + (y[i] - cy) * (y[i] - cy)).sqrt();
                if r < result && (0..n).all(|l| (x[l] - cx) * (x[l] - cx) + (y[l] - cy) * (y[l] - cy) - r * r <= 1e-6) {
                    result = r;
                }
            }
        }
    }
    println!("{}", result);
}
