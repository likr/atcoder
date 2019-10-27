use std::f64::consts::PI;


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
        a: f64,
        b: f64,
        x: f64,
    }
    let area = x / a;
    let half_pi = PI / 2.;
    let error = 1e-8;

    let mut theta_min = 0.;
    let mut theta_max = PI / 2.;
    loop {
        let theta = (theta_max + theta_min) / 2.;
        // println!("{} {}", theta_min, theta_max);
        // println!(" {}", theta);
        let px = b * theta.cos();
        let py = b * theta.sin();
        let l2b = a * ((theta + half_pi).sin() - theta.tan() * (theta + half_pi).cos());
        let l1x = py / (theta + half_pi).tan();
        let l2x = (py - l2b) / theta.tan();
        // println!("px={}, py={}", px, py);
        // println!("l1x={}, l2x={}, l2b={}", l1x, l2x, l2b);
        let area_theta = if l1x > l2x {
            let d = (l1x * l1x + py * py).sqrt();
            b * d / 2.
        } else {
            let dx = l2x - a * (theta + half_pi).cos();
            let dy = py - a * (theta + half_pi).sin();
            let d = (dx * dx + dy * dy).sqrt();
            a * (b + d) / 2.
        };
        // println!("{} {}", area, area_theta);
        if area_theta <= area {
            if area - area_theta < error {
                break;
            }
            theta_min = theta;
        } else {
            theta_max = theta;
        }
    }
    let theta = PI / 2. - (theta_max + theta_min) / 2.;
    println!("{}", theta / PI * 180.);
}
