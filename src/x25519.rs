//! y^2 = x^3 + Ax^2 + x (mod P)

use crate::math::ModInverse;

/// We mod by this. Prime field (2^255 - 19 normally)
const P: u128 = 97;
/// Generator Point (Base Point) (x-coordinate)
const G: u128 = 3;
/// The curve Coeffecient non-singular if (A^2 - 4) % P != 0
const A: u128 = 5;

fn double_point(x: u128) -> u128 {
    let x2 = (x * x) % P;
    let numerator = ((x2 + P - 1) % P).pow(2) % P;
    let denominator = (4 * x % P * (x2 + A * x + 1) % P) % P;
    (numerator * denominator.mod_inverse(P)) % P
}

fn differential_add(curr: u128, next: u128, base: u128) -> u128 {
    if curr == 0 {
        return next;
    }
    if next == 0 {
        return curr;
    }
    if curr == next {
        return double_point(curr);
    }

    let numerator = ((curr * next + P - 1) % P).pow(2) % P;

    let x_diff = if curr > next {
        curr - next
    } else {
        next - curr
    };
    let denominator = (base * x_diff.pow(2)) % P;
    (numerator * denominator.mod_inverse(P)) % P
}

fn scalar_mult(scalar: u128, base: u128) -> u128 {
    assert!(scalar != 0);

    let mut curr = base;
    let mut next = double_point(base);

    // Subtract 1 as we start at step 1 rather than 0
    for bit in (0..highest_bit(scalar) - 1).rev() {
        if (scalar >> bit) & 1 == 1 {
            curr = differential_add(curr, next, base);
            next = double_point(next);
        } else {
            next = differential_add(curr, next, base);
            curr = double_point(curr);
        }
    }

    curr
}

struct X97 {
    public: u128,
    private: u128,
}

impl X97 {
    pub fn new(private: u128) -> Self {
        Self {
            public: scalar_mult(private, G),
            private,
        }
    }

    pub fn generate_shared_secret(&self, other_public: u128) -> u128 {
        scalar_mult(self.private, other_public)
    }
}

#[inline(always)]
fn highest_bit(scalar: u128) -> u32 {
    128 - scalar.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alicebob() {
        let alice = X97::new(20);
        let bob = X97::new(69);

        let shared_alice = alice.generate_shared_secret(bob.public);
        let shared_bob = bob.generate_shared_secret(alice.public);

        assert_eq!(shared_bob, shared_alice)
    }

    #[test]
    fn alicebob_raw() {
        let alice_secret = 10;
        let alice_public = scalar_mult(alice_secret, G);

        let bob_secret = 15;
        let bob_public = scalar_mult(bob_secret, G);

        let shared_by_alice = scalar_mult(alice_secret, bob_public);
        let shared_by_bob = scalar_mult(bob_secret, alice_public);
        assert_eq!(shared_by_bob, shared_by_alice)
    }

    #[test]
    fn test_double_point() {
        assert_eq!(double_point(3), 61);
        assert_ne!(double_point(3), 60);
    }

    #[test]
    fn test_curve_non_singular() {
        let discriminant = (A.pow(2) - 4) % P;
        let normalised_discriminant = (discriminant + P) % P;
        assert_ne!(normalised_discriminant, 0);
    }

    #[test]
    fn test_g_on_curve() {
        let rhs = (G.pow(3) + A * G.pow(2) + G) % P;
        let on_curve = (0..P).any(|y| y.pow(2) % P == rhs);
        assert!(on_curve)
    }
}
