use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}



fn main() {
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
    println!("Hello, world!");

    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 5, im: 2 };
    assert_eq!(x, y);

    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);

    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
}

// trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
//     fn add(self, rhs: Self) -> Self {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im,
//         }
//     }
// }

// impl<T> Add for Complex<T>
// where
//     T: Add<Output = T>,
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im,
//         }
//     }
// }

// maximally generic (not so different that previous one)
// impl<L, R> Add<Complex<R>> for Complex<L>
// where
//     L : Add<R>,
// {
//     type Output = Complex<L::Output>;
//     fn add(self, rhs: Complex<R>) -> Self::Output {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im,
//         }
//     }
// }

// trait Neg {
//     type Output;
//     fn neg(self) -> Self::Output;
// }

// trait Not {
//     type Output;
//     fn not(self) -> Self::Output;
// }

use std::ops::Neg;

impl<T> Neg for Complex<T>
where 
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// trait AddAssign<Rhs = Self> {
//     fn add_assign(&mut self, rhs: Rhs);
// }
// x += y => x.add_assign(y)

use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// trait PartialEq<Rhs = Self>
// where
//     Rhs: ?Sized,
// {
//     fn eq(&self, other: &Rhs) -> bool;
//     fn ne(&self, other: &Rhs) -> bool {
//         !self.eq(other)
//     }
// }

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}