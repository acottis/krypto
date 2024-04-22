pub fn carmichael(n: u64) -> u64 {
    let mut k = 1;
    if n == 1 {
        return k;
    }
    let mut coprimes = (1..n).filter(|i| u64::gcd(*i, n) == 1);

    while !coprimes.all(|coprime| coprime.pow_mod(k, n) == 1) {
        k += 1;
    }

    return k;
}

pub fn phi(n: u64) -> u64 {
    let mut count = 1;
    for i in 1..(n - 1) {
        if u64::gcd(i, n) == 1 {
            count += 1
        }
    }
    count
}

pub trait ModulusInverse {
    fn mod_inverse(self, modulus: Self) -> Self;
}

macro_rules! impl_mod_inverse {
    ($ty:ty) => {
        impl ModulusInverse for $ty {
            fn mod_inverse(self, modulus: Self) -> Self {
                let mut result = 1;
                while (self * result) % modulus != 1 {
                    result += 1;
                }
                result
            }
        }
    };
}

impl_mod_inverse!(i8);
impl_mod_inverse!(i16);
impl_mod_inverse!(i32);
impl_mod_inverse!(i64);
impl_mod_inverse!(i128);
impl_mod_inverse!(u8);
impl_mod_inverse!(u16);
impl_mod_inverse!(u32);
impl_mod_inverse!(u64);
impl_mod_inverse!(u128);

pub trait GreatestCommonDivisor {
    fn gcd(self, other: Self) -> Self;
}

macro_rules! impl_gcd {
    ($ty:ty) => {
        impl GreatestCommonDivisor for $ty {
            fn gcd(self, other: Self) -> Self {
                if self == 0 {
                    return other;
                }
                return Self::gcd(other % self, self);
            }
        }
    };
}

impl_gcd!(i8);
impl_gcd!(i16);
impl_gcd!(i32);
impl_gcd!(i64);
impl_gcd!(i128);
impl_gcd!(u8);
impl_gcd!(u16);
impl_gcd!(u32);
impl_gcd!(u64);
impl_gcd!(u128);

pub trait LowestCommonMultiple {
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_lcm {
    ($ty:ty) => {
        impl LowestCommonMultiple for $ty {
            fn lcm(self, other: Self) -> Self {
                (self * other) / Self::gcd(self, other)
            }
        }
    };
}

impl_lcm!(i8);
impl_lcm!(i16);
impl_lcm!(i32);
impl_lcm!(i64);
impl_lcm!(i128);
impl_lcm!(u8);
impl_lcm!(u16);
impl_lcm!(u32);
impl_lcm!(u64);
impl_lcm!(u128);

pub trait PhiPrime {
    fn phi_prime(self) -> Self;
}

macro_rules! impl_phi_prime {
    ($ty:ty) => {
        impl PhiPrime for $ty {
            #[inline(always)]
            fn phi_prime(self) -> Self {
                self - 1
            }
        }
    };
}

impl_phi_prime!(i8);
impl_phi_prime!(i16);
impl_phi_prime!(i32);
impl_phi_prime!(i64);
impl_phi_prime!(i128);
impl_phi_prime!(u8);
impl_phi_prime!(u16);
impl_phi_prime!(u32);
impl_phi_prime!(u64);
impl_phi_prime!(u128);

pub trait PowMod {
    fn pow_mod(self, exp: Self, modulus: Self) -> Self;
}

// Optimised version of the below
// let mut result = self;
// for i in 1..exp {
//     result = (result * self) % modulus
// }
// result
macro_rules! impl_pow_mod {
    ($ty:ty) => {
        impl PowMod for $ty {
            fn pow_mod(mut self, mut exp: Self, modulus: Self) -> Self {
                let mut result = 1;
                while exp > 0 {
                    if exp % 2 == 1 {
                        result = (result * self) % modulus;
                    }
                    exp >>= 1;
                    self = (self * self) % modulus;
                }

                result
            }
        }
    };
}

impl_pow_mod!(i8);
impl_pow_mod!(i16);
impl_pow_mod!(i32);
impl_pow_mod!(i64);
impl_pow_mod!(i128);
impl_pow_mod!(u8);
impl_pow_mod!(u16);
impl_pow_mod!(u32);
impl_pow_mod!(u64);
impl_pow_mod!(u128);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pow_mod_u64() {
        assert_eq!(u64::pow_mod(2, 4, 5), 1);
        assert_eq!(u64::pow_mod(4, 16, 23), 12);
        assert_eq!(u64::pow_mod(69, 69, 43), 8);
    }
    #[test]
    fn pow_mod_u32() {
        assert_eq!(u32::pow_mod(2, 4, 5), 1);
        assert_eq!(u32::pow_mod(4, 16, 23), 12);
        assert_eq!(u32::pow_mod(69, 69, 43), 8);
    }

    #[test]
    fn test_carmichael() {
        assert!(carmichael(1) == 1);
        assert!(carmichael(2) == 1);
        assert!(carmichael(3) == 2);
        assert!(carmichael(5) == 4);
        assert!(carmichael(10) == 4);
        assert!(carmichael(35) == 12);
    }

    #[test]
    fn test_gcd() {
        // Test cases with known GCD values
        assert_eq!(u64::gcd(48, 18), 6);
        assert_eq!(u64::gcd(252, 105), 21);
        assert_eq!(u64::gcd(35, 14), 7);
        assert_eq!(u64::gcd(81, 27), 27);

        // Test cases with one or both inputs as zero
        assert_eq!(u64::gcd(0, 7), 7);
        assert_eq!(u64::gcd(12, 0), 12);
        assert_eq!(u64::gcd(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        // Test cases with known LCM values
        assert_eq!(u64::lcm(12, 18), 36);
        assert_eq!(u64::lcm(15, 20), 60);
        assert_eq!(u64::lcm(7, 9), 63);
        assert_eq!(u64::lcm(6, 8), 24);
        assert_eq!(u64::lcm(60, 52), 780);

        // Test cases with one or both inputs as zero
        //        assert_eq!(lcm(0, 7), 0);
        //        assert_eq!(lcm(12, 0), 0);
        //        assert_eq!(lcm(0, 0), 0);
    }

    #[test]
    fn test_phi() {
        // Test cases with known phi(n) values
        assert_eq!(phi(1), 1);
        assert_eq!(phi(2), 1);
        assert_eq!(phi(5), 4);
        assert_eq!(phi(10), 4);
        assert_eq!(phi(12), 4);
        assert_eq!(phi(100), 40);

        // Test cases with prime numbers
        assert_eq!(phi(3), 2);
        assert_eq!(phi(7), 6);
        assert_eq!(phi(11), 10);
        assert_eq!(phi(13), 12);
        assert_eq!(phi(17), 16);
    }

    #[test]
    fn test_mod_inverse() {
        // Test cases with known modular inverse values
        assert_eq!(3.mod_inverse(11), 4);
        assert_eq!(7.mod_inverse(13), 2);
        assert_eq!(9.mod_inverse(23), 18);
    }
}
