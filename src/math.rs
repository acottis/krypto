pub fn carmichael(n: u64) -> u32 {
    let mut k = 1;
    if n == 1 {
        return k;
    }
    let mut coprimes = (1..n).filter(|i| gcd(*i, n) == 1);

    while !coprimes.all(|coprime| coprime.pow(k) % n == 1) {
        k += 1;
    }

    return k;
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

pub fn phi(n: u64) -> u64 {
    let mut count = 1;
    for i in 1..(n - 1) {
        if gcd(i, n) == 1 {
            count += 1
        }
    }
    count
}

pub fn mod_inverse(element: u64, modulus: u64) -> u64 {
    let mut answer = 1;
    while (element * answer) % modulus != 1 {
        answer += 1;
    }
    answer
}

#[inline(always)]
pub fn phi_prime(prime: u64) -> u64 {
    prime - 1
}

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
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(252, 105), 21);
        assert_eq!(gcd(35, 14), 7);
        assert_eq!(gcd(81, 27), 27);

        // Test cases with one or both inputs as zero
        assert_eq!(gcd(0, 7), 7);
        assert_eq!(gcd(12, 0), 12);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        // Test cases with known LCM values
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(15, 20), 60);
        assert_eq!(lcm(7, 9), 63);
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(60, 52), 780);

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
        assert_eq!(mod_inverse(3, 11), 4);
        assert_eq!(mod_inverse(7, 13), 2);
        assert_eq!(mod_inverse(9, 23), 18);
    }
}
