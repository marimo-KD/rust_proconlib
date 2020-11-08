use super::alge_struct::alge;
use super::static_modint::static_modint;
use cargo_snippet::snippet;
#[snippet]
#[macro_use]
#[snippet(include = "alge")]
#[snippet(include = "static_modint")]
mod ntt {
    use super::alge::{One, Zero};
    use super::static_modint::{Mod, Modint};

    pub trait NTTableMod: Mod {
        const PRIMITIVE_ROOT: u64;
    }
    fn _ntt<M: NTTableMod>(a: &mut [Modint<M>], g: Modint<M>) {
        // Stockhamの変種とおもわれる
        // https://satanic0258.github.io/snippets/math/FFT.html
        let n = a.len();
        assert!(n.is_power_of_two());
        let mask = n - 1;
        let lgn = n.trailing_zeros();
        let mut a = a; // あとのbとライフタイムを揃える
        let mut b = vec![Modint::zero(); n].into_boxed_slice();
        let mut b: &mut [Modint<M>] = &mut b;
        let root = g.pow((M::M - 1) / n as u64);

        for _i in (0..lgn).rev() {
            std::mem::swap(&mut a, &mut b);
            let i = 1 << _i;
            let d = root.pow(i as u64);
            let mut w = Modint::one();
            for j in (0..n).step_by(i) {
                for k in 0..i {
                    a[j + k] = b[(j * 2 & mask) + k] + b[((j * 2 + i) & mask) + k] * w;
                }
                w = w * d;
            }
        }
        if lgn % 2 == 1 {
            b.copy_from_slice(a);
        }
    }
    pub fn ntt<M: NTTableMod>(a: &mut [Modint<M>]) {
        _ntt(a, Modint::new(M::PRIMITIVE_ROOT));
    }
    pub fn intt<M: NTTableMod>(a: &mut [Modint<M>]) {
        _ntt(a, Modint::new(M::PRIMITIVE_ROOT).inv());
        let n = a.len();
        let ninv = Modint::from(n).inv();
        a.iter_mut().for_each(|x| {
            *x *= ninv;
        });
    }
    pub fn convolution<M: NTTableMod>(
        mut x: Vec<Modint<M>>,
        mut y: Vec<Modint<M>>,
    ) -> Vec<Modint<M>> {
        let n = x.len() + y.len() - 1;
        let sz = n.next_power_of_two();
        x.resize(sz, Modint::zero());
        y.resize(sz, Modint::zero());
        ntt(&mut x);
        ntt(&mut y);
        x.iter_mut().zip(y).for_each(|(x,y)|{
            *x *= y;
        });
        intt(&mut x);
        x.truncate(n);
        x
    }
    #[macro_export]
    macro_rules! define_nttable_mod {
        ($struct_name:ident, $modulo:expr, $root: expr) => {
            define_mod!($struct_name, $modulo);
            impl NTTableMod for $struct_name {
                const PRIMITIVE_ROOT: u64 = $root;
            }
        };
    }

    define_nttable_mod!(P998244353, 998244353, 3);
    define_nttable_mod!(P1224736769, 1224736769, 3);
    define_nttable_mod!(P469762049, 469762049, 3);
    define_nttable_mod!(P167772161, 167772161, 3);
}
