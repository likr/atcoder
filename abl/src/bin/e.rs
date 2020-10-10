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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

type Mint = ModInt998244353;

struct S;
impl Monoid for S {
    type S = (Mint, Mint);

    fn identity() -> Self::S {
        (0.into(), 0.into())
    }

    fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

struct F;
impl MapMonoid for F {
    type M = S;
    type F = Mint;

    fn identity_map() -> Self::F {
        1.into()
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x.1 * f, x.1)
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        debug!(f, g);
        f * g
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        lrd: [(Usize1, Usize1, usize); q],
    }

    let mut segtree = LazySegtree::<F>::new(n);
    let mut b = Mint::new(1);
    for i in 0..n {
        segtree.set(i, (b, b));
        b = b * Mint::new(10);
    }
    for &(li, ri, di) in &lrd {
        let left = n - ri - 1;
        let right = n - li;
        debug!(left, right);
        segtree.apply_range(left, right, Mint::new(di));
        println!("{}", segtree.all_prod().0);
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_bit {
    // Skipped:
    //
    // - `bsf` = `__builtin_ctz`: is equivalent to `{integer}::trailing_zeros`

    #[allow(dead_code)]
    pub(crate) fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn ceil_pow2() {
            // https://github.com/atcoder/ac-library/blob/2088c8e2431c3f4d29a2cfabc6529fe0a0586c48/test/unittest/bit_test.cpp
            assert_eq!(0, super::ceil_pow2(0));
            assert_eq!(0, super::ceil_pow2(1));
            assert_eq!(1, super::ceil_pow2(2));
            assert_eq!(2, super::ceil_pow2(3));
            assert_eq!(2, super::ceil_pow2(4));
            assert_eq!(3, super::ceil_pow2(5));
            assert_eq!(3, super::ceil_pow2(6));
            assert_eq!(3, super::ceil_pow2(7));
            assert_eq!(3, super::ceil_pow2(8));
            assert_eq!(4, super::ceil_pow2(9));
            assert_eq!(30, super::ceil_pow2(1 << 30));
            assert_eq!(31, super::ceil_pow2((1 << 30) + 1));

            assert_eq!(32, super::ceil_pow2(u32::max_value()));
        }
    }
}
pub mod internal_type_traits {
    use std::{
        fmt,
        iter::{Product, Sum},
        ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        },
    };

    // Skipped:
    //
    // - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
    // - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
    // - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

    /// Corresponds to `std::is_integral` in C++.
    // We will remove unnecessary bounds later.
    //
    // Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
    // same way as the original ACL.
    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Not<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + Shl<Output = Self>
        + Shr<Output = Self>
        + ShlAssign
        + ShrAssign
        + fmt::Display
        + fmt::Debug
        + fmt::Binary
        + fmt::Octal
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
    {
    }

    /// Class that has additive identity element
    pub trait Zero {
        /// The additive identity element
        fn zero() -> Self;
    }

    /// Class that has multiplicative identity element
    pub trait One {
        /// The multiplicative identity element
        fn one() -> Self;
    }

    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
pub mod lazysegtree {
    use crate::internal_bit::ceil_pow2;
    use crate::Monoid;

    pub trait MapMonoid {
        type M: Monoid;
        type F: Clone;
        // type S = <Self::M as Monoid>::S;
        fn identity_element() -> <Self::M as Monoid>::S {
            Self::M::identity()
        }
        fn binary_operation(
            a: &<Self::M as Monoid>::S,
            b: &<Self::M as Monoid>::S,
        ) -> <Self::M as Monoid>::S {
            Self::M::binary_operation(a, b)
        }
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    impl<F: MapMonoid> Default for LazySegtree<F> {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoid> LazySegtree<F> {
        pub fn new(n: usize) -> Self {
            vec![F::identity_element(); n].into()
        }
    }
    impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for LazySegtree<F> {
        fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::identity_element(); 2 * size];
            let lz = vec![F::identity_map(); size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = LazySegtree {
                n,
                size,
                log,
                d,
                lz,
            };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }

    impl<F: MapMonoid> LazySegtree<F> {
        pub fn set(&mut self, mut p: usize, x: <F::M as Monoid>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&mut self, mut p: usize) -> <F::M as Monoid>::S {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p].clone()
        }

        pub fn prod(&mut self, mut l: usize, mut r: usize) -> <F::M as Monoid>::S {
            assert!(l <= r && r <= self.n);
            if l == r {
                return F::identity_element();
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sml = F::identity_element();
            let mut smr = F::identity_element();
            while l < r {
                if l & 1 != 0 {
                    sml = F::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = F::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            F::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> <F::M as Monoid>::S {
            self.d[1].clone()
        }

        pub fn apply(&mut self, mut p: usize, f: F::F) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
        pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F::F) {
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let l2 = l;
                let r2 = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
        where
            G: Fn(<F::M as Monoid>::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(g(F::identity_element()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            for i in (1..=self.log).rev() {
                self.push(l >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !g(F::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        self.push(l);
                        l *= 2;
                        let res = F::binary_operation(&sm, &self.d[l]);
                        if g(res.clone()) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = F::binary_operation(&sm, &self.d[l]);
                l += 1;
                //while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
        where
            G: Fn(<F::M as Monoid>::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(g(F::identity_element()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            for i in (1..=self.log).rev() {
                self.push((r - 1) >> i);
            }
            let mut sm = F::identity_element();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 != 0 {
                    r >>= 1;
                }
                if !g(F::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        self.push(r);
                        r = 2 * r + 1;
                        let res = F::binary_operation(&self.d[r], &sm);
                        if g(res.clone()) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = F::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }
    }

    pub struct LazySegtree<F>
    where
        F: MapMonoid,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<<F::M as Monoid>::S>,
        lz: Vec<F::F>,
    }
    impl<F> LazySegtree<F>
    where
        F: MapMonoid,
    {
        fn update(&mut self, k: usize) {
            self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F::F) {
            self.d[k] = F::mapping(&f, &self.d[k]);
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }

    // TODO is it useful?
    use std::fmt::{Debug, Error, Formatter, Write};
    impl<F> Debug for LazySegtree<F>
    where
        F: MapMonoid,
        F::F: Debug,
        <F::M as Monoid>::S: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.log {
                for j in 0..1 << i {
                    f.write_fmt(format_args!(
                        "{:?}[{:?}]\t",
                        self.d[(1 << i) + j],
                        self.lz[(1 << i) + j]
                    ))?;
                }
                f.write_char('\n')?;
            }
            for i in 0..self.size {
                f.write_fmt(format_args!("{:?}\t", self.d[self.size + i]))?;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::{LazySegtree, MapMonoid, Max};

        struct MaxAdd;
        impl MapMonoid for MaxAdd {
            type M = Max<i32>;
            type F = i32;

            fn identity_map() -> Self::F {
                0
            }

            fn mapping(&f: &i32, &x: &i32) -> i32 {
                f + x
            }

            fn composition(&f: &i32, &g: &i32) -> i32 {
                f + g
            }
        }

        #[test]
        fn test_max_add_lazy_segtree() {
            let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            let n = base.len();
            let mut segtree: LazySegtree<MaxAdd> = base.clone().into();
            check_segtree(&base, &mut segtree);

            let mut segtree = LazySegtree::<MaxAdd>::new(n);
            let mut internal = vec![i32::min_value(); n];
            for i in 0..n {
                segtree.set(i, base[i]);
                internal[i] = base[i];
                check_segtree(&internal, &mut segtree);
            }

            segtree.set(6, 5);
            internal[6] = 5;
            check_segtree(&internal, &mut segtree);

            segtree.apply(5, 1);
            internal[5] += 1;
            check_segtree(&internal, &mut segtree);

            segtree.set(6, 0);
            internal[6] = 0;
            check_segtree(&internal, &mut segtree);

            segtree.apply_range(3, 8, 2);
            internal[3..8].iter_mut().for_each(|e| *e += 2);
            check_segtree(&internal, &mut segtree);
        }

        //noinspection DuplicatedCode
        fn check_segtree(base: &[i32], segtree: &mut LazySegtree<MaxAdd>) {
            let n = base.len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                assert_eq!(segtree.get(i), base[i]);
            }
            for i in 0..=n {
                for j in i..=n {
                    assert_eq!(
                        segtree.prod(i, j),
                        base[i..j].iter().max().copied().unwrap_or(i32::min_value())
                    );
                }
            }
            assert_eq!(
                segtree.all_prod(),
                base.iter().max().copied().unwrap_or(i32::min_value())
            );
            for k in 0..=10 {
                let f = |x| x < k;
                for i in 0..=n {
                    assert_eq!(
                        Some(segtree.max_right(i, f)),
                        (i..=n)
                            .filter(|&j| f(base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .max()
                    );
                }
                for j in 0..=n {
                    assert_eq!(
                        Some(segtree.min_left(j, f)),
                        (0..=j)
                            .filter(|&i| f(base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .min()
                    );
                }
            }
        }
    }
}
pub mod segtree {
    use crate::internal_bit::ceil_pow2;
    use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
    use std::cmp::{max, min};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};

    // TODO Should I split monoid-related traits to another module?
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }

    pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Max<S>
    where
        S: Copy + Ord + BoundedBelow,
    {
        type S = S;
        fn identity() -> Self::S {
            S::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            max(*a, *b)
        }
    }

    pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Min<S>
    where
        S: Copy + Ord + BoundedAbove,
    {
        type S = S;
        fn identity() -> Self::S {
            S::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            min(*a, *b)
        }
    }

    pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Additive<S>
    where
        S: Copy + Add<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Multiplicative<S>
    where
        S: Copy + Mul<Output = S> + One,
    {
        type S = S;
        fn identity() -> Self::S {
            S::one()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a * *b
        }
    }

    impl<M: Monoid> Default for Segtree<M> {
        fn default() -> Self {
            Segtree::new(0)
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn new(n: usize) -> Segtree<M> {
            vec![M::identity(); n].into()
        }
    }
    impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
        fn from(v: Vec<M::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![M::identity(); 2 * size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = Segtree { n, size, log, d };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn set(&mut self, mut p: usize, x: M::S) {
            assert!(p < self.n);
            p += self.size;
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);
            self.d[p + self.size].clone()
        }

        pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
            assert!(l <= r && r <= self.n);
            let mut sml = M::identity();
            let mut smr = M::identity();
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = M::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            M::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> M::S {
            self.d[1].clone()
        }

        pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&M::identity()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            let mut sm = M::identity();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&M::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        l *= 2;
                        let res = M::binary_operation(&sm, &self.d[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = M::binary_operation(&sm, &self.d[l]);
                l += 1;
                // while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&M::identity()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            let mut sm = M::identity();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&M::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = M::binary_operation(&self.d[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = M::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }

        fn update(&mut self, k: usize) {
            self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
    }

    // Maybe we can use this someday
    // ```
    // for i in 0..=self.log {
    //     for j in 0..1 << i {
    //         print!("{}\t", self.d[(1 << i) + j]);
    //     }
    //     println!();
    // }
    // ```

    pub struct Segtree<M>
    where
        M: Monoid,
    {
        // variable name is _n in original library
        n: usize,
        size: usize,
        log: usize,
        d: Vec<M::S>,
    }

    #[cfg(test)]
    mod tests {
        use crate::segtree::Max;
        use crate::Segtree;

        #[test]
        fn test_max_segtree() {
            let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            let n = base.len();
            let segtree: Segtree<Max<_>> = base.clone().into();
            check_segtree(&base, &segtree);

            let mut segtree = Segtree::<Max<_>>::new(n);
            let mut internal = vec![i32::min_value(); n];
            for i in 0..n {
                segtree.set(i, base[i]);
                internal[i] = base[i];
                check_segtree(&internal, &segtree);
            }

            segtree.set(6, 5);
            internal[6] = 5;
            check_segtree(&internal, &segtree);

            segtree.set(6, 0);
            internal[6] = 0;
            check_segtree(&internal, &segtree);
        }

        //noinspection DuplicatedCode
        fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
            let n = base.len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                assert_eq!(segtree.get(i), base[i]);
            }
            for i in 0..=n {
                for j in i..=n {
                    assert_eq!(
                        segtree.prod(i, j),
                        base[i..j].iter().max().copied().unwrap_or(i32::min_value())
                    );
                }
            }
            assert_eq!(
                segtree.all_prod(),
                base.iter().max().copied().unwrap_or(i32::min_value())
            );
            for k in 0..=10 {
                let f = |&x: &i32| x < k;
                for i in 0..=n {
                    assert_eq!(
                        Some(segtree.max_right(i, f)),
                        (i..=n)
                            .filter(|&j| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .max()
                    );
                }
                for j in 0..=n {
                    assert_eq!(
                        Some(segtree.min_left(j, f)),
                        (0..=j)
                            .filter(|&i| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .min()
                    );
                }
            }
        }
    }
}
use lazysegtree::*;
use segtree::*;
//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_math {
    // remove this after dependencies has been added
    #![allow(dead_code)]
    use std::mem::swap;

    /// # Arguments
    /// * `m` `1 <= m`
    ///
    /// # Returns
    /// x mod m
    /* const */
    pub(crate) fn safe_mod(mut x: i64, m: i64) -> i64 {
        x %= m;
        if x < 0 {
            x += m;
        }
        x
    }

    /// Fast modular by barrett reduction
    /// Reference: https://en.wikipedia.org/wiki/Barrett_reduction
    /// NOTE: reconsider after Ice Lake
    pub(crate) struct Barrett {
        pub(crate) _m: u32,
        pub(crate) im: u64,
    }

    impl Barrett {
        /// # Arguments
        /// * `m` `1 <= m`
        /// (Note: `m <= 2^31` should also hold, which is undocumented in the original library.
        /// See the [pull reqeust commment](https://github.com/rust-lang-ja/ac-library-rs/pull/3#discussion_r484661007)
        /// for more details.)
        pub(crate) fn new(m: u32) -> Barrett {
            Barrett {
                _m: m,
                im: (-1i64 as u64 / m as u64).wrapping_add(1),
            }
        }

        /// # Returns
        /// `m`
        pub(crate) fn umod(&self) -> u32 {
            self._m
        }

        /// # Parameters
        /// * `a` `0 <= a < m`
        /// * `b` `0 <= b < m`
        ///
        /// # Returns
        /// a * b % m
        #[allow(clippy::many_single_char_names)]
        pub(crate) fn mul(&self, a: u32, b: u32) -> u32 {
            // [1] m = 1
            // a = b = im = 0, so okay

            // [2] m >= 2
            // im = ceil(2^64 / m)
            // -> im * m = 2^64 + r (0 <= r < m)
            // let z = a*b = c*m + d (0 <= c, d < m)
            // a*b * im = (c*m + d) * im = c*(im*m) + d*im = c*2^64 + c*r + d*im
            // c*r + d*im < m * m + m * im < m * m + 2^64 + m <= 2^64 + m * (m + 1) < 2^64 * 2
            // ((ab * im) >> 64) == c or c + 1
            let mut z = a as u64;
            z *= b as u64;
            let x = (((z as u128) * (self.im as u128)) >> 64) as u64;
            let mut v = z.wrapping_sub(x.wrapping_mul(self._m as u64)) as u32;
            if self._m <= v {
                v = v.wrapping_add(self._m);
            }
            v
        }
    }

    /// # Parameters
    /// * `n` `0 <= n`
    /// * `m` `1 <= m`
    ///
    /// # Returns
    /// `(x ** n) % m`
    /* const */
    #[allow(clippy::many_single_char_names)]
    pub(crate) fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
        if m == 1 {
            return 0;
        }
        let _m = m as u32;
        let mut r: u64 = 1;
        let mut y: u64 = safe_mod(x, m as i64) as u64;
        while n != 0 {
            if (n & 1) > 0 {
                r = (r * y) % (_m as u64);
            }
            y = (y * y) % (_m as u64);
            n >>= 1;
        }
        r as i64
    }

    /// Reference:
    /// M. Forisek and J. Jancina,
    /// Fast Primality Testing for Integers That Fit into a Machine Word
    ///
    /// # Parameters
    /// * `n` `0 <= n`
    /* const */
    pub(crate) fn is_prime(n: i32) -> bool {
        let n = n as i64;
        match n {
            _ if n <= 1 => return false,
            2 | 7 | 61 => return true,
            _ if n % 2 == 0 => return false,
            _ => {}
        }
        let mut d = n - 1;
        while d % 2 == 0 {
            d /= 2;
        }
        for &a in &[2, 7, 61] {
            let mut t = d;
            let mut y = pow_mod(a, t, n as i32);
            while t != n - 1 && y != 1 && y != n - 1 {
                y = y * y % n;
                t <<= 1;
            }
            if y != n - 1 && t % 2 == 0 {
                return false;
            }
        }
        true
    }

    // omitted
    // template <int n> constexpr bool is_prime = is_prime_constexpr(n);

    /// # Parameters
    /// * `b` `1 <= b`
    ///
    /// # Returns
    /// (g, x) s.t. g = gcd(a, b), xa = g (mod b), 0 <= x < b/g
    /* const */
    #[allow(clippy::many_single_char_names)]
    pub(crate) fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
        let a = safe_mod(a, b);
        if a == 0 {
            return (b, 0);
        }

        // Contracts:
        // [1] s - m0 * a = 0 (mod b)
        // [2] t - m1 * a = 0 (mod b)
        // [3] s * |m1| + t * |m0| <= b
        let mut s = b;
        let mut t = a;
        let mut m0 = 0;
        let mut m1 = 1;

        while t != 0 {
            let u = s / t;
            s -= t * u;
            m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

            // [3]:
            // (s - t * u) * |m1| + t * |m0 - m1 * u|
            // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
            // = s * |m1| + t * |m0| <= b

            swap(&mut s, &mut t);
            swap(&mut m0, &mut m1);
        }
        // by [3]: |m0| <= b/g
        // by g != b: |m0| < b/g
        if m0 < 0 {
            m0 += b / s;
        }
        (s, m0)
    }

    /// Compile time (currently not) primitive root
    /// @param m must be prime
    /// @return primitive root (and minimum in now)
    /* const */
    pub(crate) fn primitive_root(m: i32) -> i32 {
        match m {
            2 => return 1,
            167_772_161 => return 3,
            469_762_049 => return 3,
            754_974_721 => return 11,
            998_244_353 => return 3,
            _ => {}
        }

        let mut divs = [0; 20];
        divs[0] = 2;
        let mut cnt = 1;
        let mut x = (m - 1) / 2;
        while x % 2 == 0 {
            x /= 2;
        }
        for i in (3..std::i32::MAX).step_by(2) {
            if i as i64 * i as i64 > x as i64 {
                break;
            }
            if x % i == 0 {
                divs[cnt] = i;
                cnt += 1;
                while x % i == 0 {
                    x /= i;
                }
            }
        }
        if x > 1 {
            divs[cnt] = x;
            cnt += 1;
        }
        let mut g = 2;
        loop {
            if (0..cnt).all(|i| pow_mod(g, ((m - 1) / divs[i]) as i64, m) != 1) {
                break g as i32;
            }
            g += 1;
        }
    }
    // omitted
    // template <int m> constexpr int primitive_root = primitive_root_constexpr(m);

    #[cfg(test)]
    mod tests {
        #![allow(clippy::unreadable_literal)]
        #![allow(clippy::cognitive_complexity)]
        use crate::internal_math::{inv_gcd, is_prime, pow_mod, primitive_root, safe_mod, Barrett};
        use std::collections::HashSet;

        #[test]
        fn test_safe_mod() {
            assert_eq!(safe_mod(0, 3), 0);
            assert_eq!(safe_mod(1, 3), 1);
            assert_eq!(safe_mod(2, 3), 2);
            assert_eq!(safe_mod(3, 3), 0);
            assert_eq!(safe_mod(4, 3), 1);
            assert_eq!(safe_mod(5, 3), 2);
            assert_eq!(safe_mod(73, 11), 7);
            assert_eq!(safe_mod(2306249155046129918, 6620319213327), 1374210749525);

            assert_eq!(safe_mod(-1, 3), 2);
            assert_eq!(safe_mod(-2, 3), 1);
            assert_eq!(safe_mod(-3, 3), 0);
            assert_eq!(safe_mod(-4, 3), 2);
            assert_eq!(safe_mod(-5, 3), 1);
            assert_eq!(safe_mod(-7170500492396019511, 777567337), 333221848);
        }

        #[test]
        fn test_barrett() {
            let b = Barrett::new(7);
            assert_eq!(b.umod(), 7);
            assert_eq!(b.mul(2, 3), 6);
            assert_eq!(b.mul(4, 6), 3);
            assert_eq!(b.mul(5, 0), 0);

            let b = Barrett::new(998244353);
            assert_eq!(b.umod(), 998244353);
            assert_eq!(b.mul(2, 3), 6);
            assert_eq!(b.mul(3141592, 653589), 919583920);
            assert_eq!(b.mul(323846264, 338327950), 568012980);

            // make `z - x * self._m as u64` overflow.
            // Thanks @koba-e964 (at https://github.com/rust-lang-ja/ac-library-rs/pull/3#discussion_r484932161)
            let b = Barrett::new(2147483647);
            assert_eq!(b.umod(), 2147483647);
            assert_eq!(b.mul(1073741824, 2147483645), 2147483646);
        }

        #[test]
        fn test_pow_mod() {
            assert_eq!(pow_mod(0, 0, 1), 0);
            assert_eq!(pow_mod(0, 0, 3), 1);
            assert_eq!(pow_mod(0, 0, 723), 1);
            assert_eq!(pow_mod(0, 0, 998244353), 1);
            assert_eq!(pow_mod(0, 0, i32::max_value()), 1);

            assert_eq!(pow_mod(0, 1, 1), 0);
            assert_eq!(pow_mod(0, 1, 3), 0);
            assert_eq!(pow_mod(0, 1, 723), 0);
            assert_eq!(pow_mod(0, 1, 998244353), 0);
            assert_eq!(pow_mod(0, 1, i32::max_value()), 0);

            assert_eq!(pow_mod(0, i64::max_value(), 1), 0);
            assert_eq!(pow_mod(0, i64::max_value(), 3), 0);
            assert_eq!(pow_mod(0, i64::max_value(), 723), 0);
            assert_eq!(pow_mod(0, i64::max_value(), 998244353), 0);
            assert_eq!(pow_mod(0, i64::max_value(), i32::max_value()), 0);

            assert_eq!(pow_mod(1, 0, 1), 0);
            assert_eq!(pow_mod(1, 0, 3), 1);
            assert_eq!(pow_mod(1, 0, 723), 1);
            assert_eq!(pow_mod(1, 0, 998244353), 1);
            assert_eq!(pow_mod(1, 0, i32::max_value()), 1);

            assert_eq!(pow_mod(1, 1, 1), 0);
            assert_eq!(pow_mod(1, 1, 3), 1);
            assert_eq!(pow_mod(1, 1, 723), 1);
            assert_eq!(pow_mod(1, 1, 998244353), 1);
            assert_eq!(pow_mod(1, 1, i32::max_value()), 1);

            assert_eq!(pow_mod(1, i64::max_value(), 1), 0);
            assert_eq!(pow_mod(1, i64::max_value(), 3), 1);
            assert_eq!(pow_mod(1, i64::max_value(), 723), 1);
            assert_eq!(pow_mod(1, i64::max_value(), 998244353), 1);
            assert_eq!(pow_mod(1, i64::max_value(), i32::max_value()), 1);

            assert_eq!(pow_mod(i64::max_value(), 0, 1), 0);
            assert_eq!(pow_mod(i64::max_value(), 0, 3), 1);
            assert_eq!(pow_mod(i64::max_value(), 0, 723), 1);
            assert_eq!(pow_mod(i64::max_value(), 0, 998244353), 1);
            assert_eq!(pow_mod(i64::max_value(), 0, i32::max_value()), 1);

            assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 1), 0);
            assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 3), 1);
            assert_eq!(pow_mod(i64::max_value(), i64::max_value(), 723), 640);
            assert_eq!(
                pow_mod(i64::max_value(), i64::max_value(), 998244353),
                683296792
            );
            assert_eq!(
                pow_mod(i64::max_value(), i64::max_value(), i32::max_value()),
                1
            );

            assert_eq!(pow_mod(2, 3, 1_000_000_007), 8);
            assert_eq!(pow_mod(5, 7, 1_000_000_007), 78125);
            assert_eq!(pow_mod(123, 456, 1_000_000_007), 565291922);
        }

        #[test]
        fn test_is_prime() {
            assert!(!is_prime(0));
            assert!(!is_prime(1));
            assert!(is_prime(2));
            assert!(is_prime(3));
            assert!(!is_prime(4));
            assert!(is_prime(5));
            assert!(!is_prime(6));
            assert!(is_prime(7));
            assert!(!is_prime(8));
            assert!(!is_prime(9));

            // assert!(is_prime(57));
            assert!(!is_prime(57));
            assert!(!is_prime(58));
            assert!(is_prime(59));
            assert!(!is_prime(60));
            assert!(is_prime(61));
            assert!(!is_prime(62));

            assert!(!is_prime(701928443));
            assert!(is_prime(998244353));
            assert!(!is_prime(1_000_000_000));
            assert!(is_prime(1_000_000_007));

            assert!(is_prime(i32::max_value()));
        }

        #[test]
        fn test_is_prime_sieve() {
            let n = 1_000_000;
            let mut prime = vec![true; n];
            prime[0] = false;
            prime[1] = false;
            for i in 0..n {
                assert_eq!(prime[i], is_prime(i as i32));
                if prime[i] {
                    for j in (2 * i..n).step_by(i) {
                        prime[j] = false;
                    }
                }
            }
        }

        #[test]
        fn test_inv_gcd() {
            for &(a, b, g) in &[
                (0, 1, 1),
                (0, 4, 4),
                (0, 7, 7),
                (2, 3, 1),
                (-2, 3, 1),
                (4, 6, 2),
                (-4, 6, 2),
                (13, 23, 1),
                (57, 81, 3),
                (12345, 67890, 15),
                (-3141592 * 6535, 3141592 * 8979, 3141592),
                (i64::max_value(), i64::max_value(), i64::max_value()),
                (i64::min_value(), i64::max_value(), 1),
            ] {
                let (g_, x) = inv_gcd(a, b);
                assert_eq!(g, g_);
                let b_ = b as i128;
                assert_eq!(((x as i128 * a as i128) % b_ + b_) % b_, g as i128 % b_);
            }
        }

        #[test]
        fn test_primitive_root() {
            for &p in &[
                2,
                3,
                5,
                7,
                233,
                200003,
                998244353,
                1_000_000_007,
                i32::max_value(),
            ] {
                assert!(is_prime(p));
                let g = primitive_root(p);
                if p != 2 {
                    assert_ne!(g, 1);
                }

                let q = p - 1;
                for i in (2..i32::max_value()).take_while(|i| i * i <= q) {
                    if q % i != 0 {
                        break;
                    }
                    for &r in &[i, q / i] {
                        assert_ne!(pow_mod(g as i64, r as i64, p), 1);
                    }
                }
                assert_eq!(pow_mod(g as i64, q as i64, p), 1);

                if p < 1_000_000 {
                    assert_eq!(
                        (0..p - 1)
                            .scan(1, |i, _| {
                                *i = *i * g % p;
                                Some(*i)
                            })
                            .collect::<HashSet<_>>()
                            .len() as i32,
                        p - 1
                    );
                }
            }
        }
    }
}
pub mod modint {
    //! Structs that treat the modular arithmetic.
    //!
    //! # Major changes from the original ACL
    //!
    //! - Converted the struct names to PascalCase.
    //! - Renamed `mod` → `modulus`.
    //! - Moduli are `u32`, not `i32`.
    //! - `Id`s are `usize`, not `i32`.
    //! - The default `Id` is `0`, not `-1`.
    //! - The type of the argument of `pow` is `u64`, not `i64`.
    //! - Modints implement `FromStr` and `Display`. Modints in the original ACL don't have `operator<<` or `operator>>`.

    use crate::internal_math;
    use std::{
        cell::RefCell,
        convert::{Infallible, TryInto as _},
        fmt,
        hash::{Hash, Hasher},
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
        str::FromStr,
        thread::LocalKey,
    };

    pub type ModInt1000000007 = StaticModInt<Mod1000000007>;
    pub type ModInt998244353 = StaticModInt<Mod998244353>;
    pub type ModInt = DynamicModInt<DefaultId>;

    /// Corresponds to `atcoder::static_modint` in the original ACL.
    #[derive(Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct StaticModInt<M> {
        val: u32,
        phantom: PhantomData<fn() -> M>,
    }

    impl<M: Modulus> StaticModInt<M> {
        /// Corresponds to `atcoder::static_modint::mod` in the original ACL.
        #[inline(always)]
        pub fn modulus() -> u32 {
            M::VALUE
        }

        /// Creates a new `StaticModInt`.
        #[inline]
        pub fn new<T: RemEuclidU32>(val: T) -> Self {
            Self::raw(val.rem_euclid_u32(M::VALUE))
        }

        /// Corresponds to `atcoder::static_modint::raw` in the original ACL.
        #[inline]
        pub fn raw(val: u32) -> Self {
            Self {
                val,
                phantom: PhantomData,
            }
        }

        /// Corresponds to `atcoder::static_modint::val` in the original ACL.
        #[inline]
        pub fn val(self) -> u32 {
            self.val
        }

        /// Corresponds to `atcoder::static_modint::pow` in the original ACL.
        #[inline]
        pub fn pow(self, n: u64) -> Self {
            <Self as ModIntBase>::pow(self, n)
        }

        /// Corresponds to `atcoder::static_modint::inv` in the original ACL.
        ///
        /// # Panics
        ///
        /// Panics if the multiplicative inverse does not exist.
        #[inline]
        pub fn inv(self) -> Self {
            if M::HINT_VALUE_IS_PRIME {
                if self.val() == 0 {
                    panic!("attempt to divide by zero");
                }
                debug_assert!(
                    internal_math::is_prime(M::VALUE.try_into().unwrap()),
                    "{} is not a prime number",
                    M::VALUE,
                );
                self.pow((M::VALUE - 2).into())
            } else {
                Self::inv_for_non_prime_modulus(self)
            }
        }
    }

    impl<M: Modulus> ModIntBase for StaticModInt<M> {
        #[inline(always)]
        fn modulus() -> u32 {
            Self::modulus()
        }

        #[inline]
        fn raw(val: u32) -> Self {
            Self::raw(val)
        }

        #[inline]
        fn val(self) -> u32 {
            self.val()
        }

        #[inline]
        fn inv(self) -> Self {
            self.inv()
        }
    }

    pub trait Modulus: 'static + Copy + Eq {
        const VALUE: u32;
        const HINT_VALUE_IS_PRIME: bool;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod1000000007 {}

    impl Modulus for Mod1000000007 {
        const VALUE: u32 = 1_000_000_007;
        const HINT_VALUE_IS_PRIME: bool = true;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {
                static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod1000000007>>> = RefCell::default();
            }
            &BUTTERFLY_CACHE
        }
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod998244353 {}

    impl Modulus for Mod998244353 {
        const VALUE: u32 = 998_244_353;
        const HINT_VALUE_IS_PRIME: bool = true;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {
                static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod998244353>>> = RefCell::default();
            }
            &BUTTERFLY_CACHE
        }
    }

    pub struct ButterflyCache<M> {
        pub sum_e: Vec<StaticModInt<M>>,
        pub sum_ie: Vec<StaticModInt<M>>,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DynamicModInt<I> {
        val: u32,
        phantom: PhantomData<fn() -> I>,
    }

    impl<I: Id> DynamicModInt<I> {
        #[inline]
        pub fn modulus() -> u32 {
            I::companion_barrett().with(|bt| bt.borrow().umod())
        }

        #[inline]
        pub fn set_modulus(modulus: u32) {
            if modulus == 0 {
                panic!("the modulus must not be 0");
            }
            I::companion_barrett().with(|bt| *bt.borrow_mut() = Barrett::new(modulus))
        }

        #[inline]
        pub fn new<T: RemEuclidU32>(val: T) -> Self {
            <Self as ModIntBase>::new(val)
        }

        #[inline]
        pub fn raw(val: u32) -> Self {
            Self {
                val,
                phantom: PhantomData,
            }
        }

        #[inline]
        pub fn val(self) -> u32 {
            self.val
        }

        #[inline]
        pub fn pow(self, n: u64) -> Self {
            <Self as ModIntBase>::pow(self, n)
        }

        #[inline]
        pub fn inv(self) -> Self {
            Self::inv_for_non_prime_modulus(self)
        }
    }

    impl<I: Id> ModIntBase for DynamicModInt<I> {
        #[inline]
        fn modulus() -> u32 {
            Self::modulus()
        }

        #[inline]
        fn raw(val: u32) -> Self {
            Self::raw(val)
        }

        #[inline]
        fn val(self) -> u32 {
            self.val()
        }

        #[inline]
        fn inv(self) -> Self {
            self.inv()
        }
    }

    pub trait Id: 'static + Copy + Eq {
        // TODO: Make `internal_math::Barret` `Copy`.
        fn companion_barrett() -> &'static LocalKey<RefCell<Barrett>>;
    }

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum DefaultId {}

    impl Id for DefaultId {
        fn companion_barrett() -> &'static LocalKey<RefCell<Barrett>> {
            thread_local! {
                static BARRETT: RefCell<Barrett> = RefCell::default();
            }
            &BARRETT
        }
    }

    pub struct Barrett(internal_math::Barrett);

    impl Barrett {
        #[inline]
        pub fn new(m: u32) -> Self {
            Self(internal_math::Barrett::new(m))
        }

        #[inline]
        fn umod(&self) -> u32 {
            self.0.umod()
        }

        #[inline]
        fn mul(&self, a: u32, b: u32) -> u32 {
            self.0.mul(a, b)
        }
    }

    impl Default for Barrett {
        #[inline]
        fn default() -> Self {
            Self(internal_math::Barrett::new(998_244_353))
        }
    }

    pub trait ModIntBase:
        Default
        + FromStr
        + From<i8>
        + From<i16>
        + From<i32>
        + From<i64>
        + From<i128>
        + From<u8>
        + From<u16>
        + From<u32>
        + From<u64>
        + From<u128>
        + Copy
        + Eq
        + Hash
        + fmt::Display
        + fmt::Debug
        + Neg<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
    {
        fn modulus() -> u32;
        fn raw(val: u32) -> Self;
        fn val(self) -> u32;
        fn inv(self) -> Self;

        #[inline]
        fn new<T: RemEuclidU32>(val: T) -> Self {
            Self::raw(val.rem_euclid_u32(Self::modulus()))
        }

        #[inline]
        fn pow(self, mut n: u64) -> Self {
            let mut x = self;
            let mut r = Self::raw(1);
            while n > 0 {
                if n & 1 == 1 {
                    r *= x;
                }
                x *= x;
                n >>= 1;
            }
            r
        }
    }

    pub trait RemEuclidU32 {
        fn rem_euclid_u32(self, modulus: u32) -> u32;
    }

    macro_rules! impl_rem_euclid_u32_for_small_signed {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self as i64).rem_euclid(i64::from(modulus)) as _
                }
            }
        )*
    }
}

    impl_rem_euclid_u32_for_small_signed!(i8, i16, i32, i64, isize);

    impl RemEuclidU32 for i128 {
        #[inline]
        fn rem_euclid_u32(self, modulus: u32) -> u32 {
            self.rem_euclid(i128::from(modulus)) as _
        }
    }

    macro_rules! impl_rem_euclid_u32_for_small_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    self as u32 % modulus
                }
            }
        )*
    }
}

    macro_rules! impl_rem_euclid_u32_for_large_unsigned {
    ($($ty:tt),*) => {
        $(
            impl RemEuclidU32 for $ty {
                #[inline]
                fn rem_euclid_u32(self, modulus: u32) -> u32 {
                    (self % (modulus as $ty)) as _
                }
            }
        )*
    }
}

    impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);
    impl_rem_euclid_u32_for_large_unsigned!(u64, u128);

    #[cfg(target_pointer_width = "32")]
    impl_rem_euclid_u32_for_small_unsigned!(usize);

    #[cfg(target_pointer_width = "64")]
    impl_rem_euclid_u32_for_large_unsigned!(usize);

    trait InternalImplementations: ModIntBase {
        #[inline]
        fn inv_for_non_prime_modulus(this: Self) -> Self {
            let (gcd, x) = internal_math::inv_gcd(this.val().into(), Self::modulus().into());
            if gcd != 1 {
                panic!("the multiplicative inverse does not exist");
            }
            Self::new(x)
        }

        #[inline]
        fn default_impl() -> Self {
            Self::raw(0)
        }

        #[inline]
        fn from_str_impl(s: &str) -> Result<Self, Infallible> {
            Ok(s.parse::<i64>()
                .map(Self::new)
                .unwrap_or_else(|_| todo!("parsing as an arbitrary precision integer?")))
        }

        #[inline]
        fn hash_impl(this: &Self, state: &mut impl Hasher) {
            this.val().hash(state)
        }

        #[inline]
        fn display_impl(this: &Self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&this.val(), f)
        }

        #[inline]
        fn debug_impl(this: &Self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&this.val(), f)
        }

        #[inline]
        fn neg_impl(this: Self) -> Self {
            Self::sub_impl(Self::raw(0), this)
        }

        #[inline]
        fn add_impl(lhs: Self, rhs: Self) -> Self {
            let modulus = Self::modulus();
            let mut val = lhs.val() + rhs.val();
            if val >= modulus {
                val -= modulus;
            }
            Self::raw(val)
        }

        #[inline]
        fn sub_impl(lhs: Self, rhs: Self) -> Self {
            let modulus = Self::modulus();
            let mut val = lhs.val().wrapping_sub(rhs.val());
            if val >= modulus {
                val = val.wrapping_add(modulus)
            }
            Self::raw(val)
        }

        fn mul_impl(lhs: Self, rhs: Self) -> Self;

        #[inline]
        fn div_impl(lhs: Self, rhs: Self) -> Self {
            Self::mul_impl(lhs, rhs.inv())
        }
    }

    impl<M: Modulus> InternalImplementations for StaticModInt<M> {
        #[inline]
        fn mul_impl(lhs: Self, rhs: Self) -> Self {
            Self::raw((u64::from(lhs.val()) * u64::from(rhs.val()) % u64::from(M::VALUE)) as u32)
        }
    }

    impl<I: Id> InternalImplementations for DynamicModInt<I> {
        #[inline]
        fn mul_impl(lhs: Self, rhs: Self) -> Self {
            I::companion_barrett().with(|bt| Self::raw(bt.borrow().mul(lhs.val, rhs.val)))
        }
    }

    macro_rules! impl_basic_traits {
    () => {};
    (impl <$generic_param:ident : $generic_param_bound:tt> _ for $self:ty; $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Default for $self {
            #[inline]
            fn default() -> Self {
                Self::default_impl()
            }
        }

        impl <$generic_param: $generic_param_bound> FromStr for $self {
            type Err = Infallible;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Infallible> {
                Self::from_str_impl(s)
            }
        }

        impl<$generic_param: $generic_param_bound, V: RemEuclidU32> From<V> for $self {
            #[inline]
            fn from(from: V) -> Self {
                Self::new(from)
            }
        }

        #[allow(clippy::derive_hash_xor_eq)]
        impl<$generic_param: $generic_param_bound> Hash for $self {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Self::hash_impl(self, state)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Display for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::display_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> fmt::Debug for $self {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Self::debug_impl(self, f)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                Self::neg_impl(self)
            }
        }

        impl<$generic_param: $generic_param_bound> Neg for &'_ $self {
            type Output = $self;

            #[inline]
            fn neg(self) -> $self {
                <$self>::neg_impl(*self)
            }
        }

        impl_basic_traits!($($rest)*);
    };
}

    impl_basic_traits! {
        impl <M: Modulus> _ for StaticModInt<M> ;
        impl <I: Id     > _ for DynamicModInt<I>;
    }

    macro_rules! impl_bin_ops {
    () => {};
    (for<$generic_param:ident : $generic_param_bound:tt> <$lhs_ty:ty> ~ <$rhs_ty:ty> -> $output:ty { { $lhs_body:expr } ~ { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> Add<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs_ty) -> $output {
                <$output>::add_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Sub<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs_ty) -> $output {
                <$output>::sub_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Mul<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs_ty) -> $output {
                <$output>::mul_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl <$generic_param: $generic_param_bound> Div<$rhs_ty> for $lhs_ty {
            type Output = $output;

            #[inline]
            fn div(self, rhs: $rhs_ty) -> $output {
                <$output>::div_impl(apply($lhs_body, self), apply($rhs_body, rhs))
            }
        }

        impl_bin_ops!($($rest)*);
    };
}

    macro_rules! impl_assign_ops {
    () => {};
    (for<$generic_param:ident : $generic_param_bound:tt> <$lhs_ty:ty> ~= <$rhs_ty:ty> { _ ~= { $rhs_body:expr } } $($rest:tt)*) => {
        impl <$generic_param: $generic_param_bound> AddAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs_ty) {
                *self = *self + apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> SubAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs_ty) {
                *self = *self - apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> MulAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs_ty) {
                *self = *self * apply($rhs_body, rhs);
            }
        }

        impl <$generic_param: $generic_param_bound> DivAssign<$rhs_ty> for $lhs_ty {
            #[inline]
            fn div_assign(&mut self, rhs: $rhs_ty) {
                *self = *self / apply($rhs_body, rhs);
            }
        }

        impl_assign_ops!($($rest)*);
    };
}

    #[inline]
    fn apply<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
        f(x)
    }

    impl_bin_ops! {
        for<M: Modulus> <StaticModInt<M>     > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |x| x  } ~ { |x| x  } }
        for<M: Modulus> <StaticModInt<M>     > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |x| x  } ~ { |&x| x } }
        for<M: Modulus> <&'_ StaticModInt<M> > ~ <StaticModInt<M>     > -> StaticModInt<M>  { { |&x| x } ~ { |x| x  } }
        for<M: Modulus> <&'_ StaticModInt<M> > ~ <&'_ StaticModInt<M> > -> StaticModInt<M>  { { |&x| x } ~ { |&x| x } }
        for<I: Id     > <DynamicModInt<I>    > ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |x| x  } ~ { |x| x  } }
        for<I: Id     > <DynamicModInt<I>    > ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |x| x  } ~ { |&x| x } }
        for<I: Id     > <&'_ DynamicModInt<I>> ~ <DynamicModInt<I>    > -> DynamicModInt<I> { { |&x| x } ~ { |x| x  } }
        for<I: Id     > <&'_ DynamicModInt<I>> ~ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { { |&x| x } ~ { |&x| x } }
    }

    impl_assign_ops! {
        for<M: Modulus> <StaticModInt<M> > ~= <StaticModInt<M>     > { _ ~= { |x| x  } }
        for<M: Modulus> <StaticModInt<M> > ~= <&'_ StaticModInt<M> > { _ ~= { |&x| x } }
        for<I: Id     > <DynamicModInt<I>> ~= <DynamicModInt<I>    > { _ ~= { |x| x  } }
        for<I: Id     > <DynamicModInt<I>> ~= <&'_ DynamicModInt<I>> { _ ~= { |&x| x } }
    }

    macro_rules! impl_folding {
    () => {};
    (impl<$generic_param:ident : $generic_param_bound:tt> $trait:ident<_> for $self:ty { fn $method:ident(_) -> _ { _($unit:expr, $op:expr) } } $($rest:tt)*) => {
        impl<$generic_param: $generic_param_bound> $trait<Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl<'a, $generic_param: $generic_param_bound> $trait<&'a Self> for $self {
            #[inline]
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = &'a Self>,
            {
                iter.fold($unit, $op)
            }
        }

        impl_folding!($($rest)*);
    };
}

    impl_folding! {
        impl<M: Modulus> Sum<_>     for StaticModInt<M>  { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
        impl<M: Modulus> Product<_> for StaticModInt<M>  { fn product(_) -> _ { _(Self::raw(1), Mul::mul) } }
        impl<I: Id     > Sum<_>     for DynamicModInt<I> { fn sum(_)     -> _ { _(Self::raw(0), Add::add) } }
        impl<I: Id     > Product<_> for DynamicModInt<I> { fn product(_) -> _ { _(Self::raw(1), Mul::mul) } }
    }

    #[cfg(test)]
    mod tests {
        use crate::modint::ModInt1000000007;

        #[test]
        fn static_modint_new() {
            assert_eq!(0, ModInt1000000007::new(0u32).val);
            assert_eq!(1, ModInt1000000007::new(1u32).val);
            assert_eq!(1, ModInt1000000007::new(1_000_000_008u32).val);

            assert_eq!(0, ModInt1000000007::new(0u64).val);
            assert_eq!(1, ModInt1000000007::new(1u64).val);
            assert_eq!(1, ModInt1000000007::new(1_000_000_008u64).val);

            assert_eq!(0, ModInt1000000007::new(0usize).val);
            assert_eq!(1, ModInt1000000007::new(1usize).val);
            assert_eq!(1, ModInt1000000007::new(1_000_000_008usize).val);

            assert_eq!(0, ModInt1000000007::new(0i64).val);
            assert_eq!(1, ModInt1000000007::new(1i64).val);
            assert_eq!(1, ModInt1000000007::new(1_000_000_008i64).val);
            assert_eq!(1_000_000_006, ModInt1000000007::new(-1i64).val);
        }

        #[test]
        fn static_modint_add() {
            fn add(lhs: u32, rhs: u32) -> u32 {
                (ModInt1000000007::new(lhs) + ModInt1000000007::new(rhs)).val
            }

            assert_eq!(2, add(1, 1));
            assert_eq!(1, add(1_000_000_006, 2));
        }

        #[test]
        fn static_modint_sub() {
            fn sub(lhs: u32, rhs: u32) -> u32 {
                (ModInt1000000007::new(lhs) - ModInt1000000007::new(rhs)).val
            }

            assert_eq!(1, sub(2, 1));
            assert_eq!(1_000_000_006, sub(0, 1));
        }

        #[test]
        fn static_modint_mul() {
            fn mul(lhs: u32, rhs: u32) -> u32 {
                (ModInt1000000007::new(lhs) * ModInt1000000007::new(rhs)).val
            }

            assert_eq!(1, mul(1, 1));
            assert_eq!(4, mul(2, 2));
            assert_eq!(999_999_937, mul(100_000, 100_000));
        }

        #[test]
        fn static_modint_prime_div() {
            fn div(lhs: u32, rhs: u32) -> u32 {
                (ModInt1000000007::new(lhs) / ModInt1000000007::new(rhs)).val
            }

            assert_eq!(0, div(0, 1));
            assert_eq!(1, div(1, 1));
            assert_eq!(1, div(2, 2));
            assert_eq!(23_809_524, div(1, 42));
        }

        #[test]
        fn static_modint_sum() {
            fn sum(values: &[i64]) -> ModInt1000000007 {
                values.iter().copied().map(ModInt1000000007::new).sum()
            }

            assert_eq!(ModInt1000000007::new(-3), sum(&[-1, 2, -3, 4, -5]));
        }

        #[test]
        fn static_modint_product() {
            fn product(values: &[i64]) -> ModInt1000000007 {
                values.iter().copied().map(ModInt1000000007::new).product()
            }

            assert_eq!(ModInt1000000007::new(-120), product(&[-1, 2, -3, 4, -5]));
        }
    }
}
use modint::*;
