use num::Complex;
use proconio::input;
use std::f64::consts::PI;

fn fft(a: &mut Vec<Complex<f64>>, n: usize, sign: f64) {
    if n == 1 {
        return;
    }
    let mut a0 = vec![Complex::new(0., 0.); n / 2];
    let mut a1 = vec![Complex::new(0., 0.); n / 2];
    for i in 0..n / 2 {
        a0[i] = a[i * 2];
        a1[i] = a[i * 2 + 1];
    }
    fft(&mut a0, n / 2, sign);
    fft(&mut a1, n / 2, sign);
    let mut w = Complex::new(1., 0.);
    let w0 = (Complex::i() * 2. * PI * sign / n as f64).exp();
    for i in 0..n / 2 {
        let t = w * a1[i];
        a[i] = a0[i] + t;
        a[n / 2 + i] = a0[i] - t;
        w *= w0;
    }
}

fn main() {
    input! {
      n: usize,
      mut m: usize,
      a: [usize; n],
    }
    let a_max = a.iter().max().unwrap().clone();
    let mut l = 1;
    while l <= a_max {
        l *= 2;
    }
    l *= 2;
    let mut x = vec![Complex::new(0f64, 0.); l];
    let mut y = vec![Complex::new(0f64, 0.); l];
    for i in 0..n {
        x[a[i]].re += 1.;
        y[a[i]].re += 1.;
    }
    // println!("{:?}", x);
    fft(&mut x, l, 1.);
    fft(&mut y, l, 1.);
    for i in 0..l {
        x[i] *= y[i];
    }
    fft(&mut x, l, -1.);
    for i in 0..l {
        x[i] /= l as f64;
    }
    // println!("{:?}", x);

    let mut s = 0;
    for i in 0..l {
        let b = l - 1 - i;
        let k = x[b].re.round() as usize;
        if k != 0 {
            let k = if k > m { m } else { k };
            // println!("{} {}", b, k);
            s += k * b;
            m -= k;
            if m == 0 {
                break;
            }
        }
    }
    println!("{}", s);
}
