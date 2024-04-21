struct Rsa {
    n: u64,
    encrypt_key: u64,
    decrypt_key: u64,
}

impl Rsa {
    pub fn new(prime_p: u64, prime_q: u64) -> Self {
        let n = prime_p * prime_q;
        // let phi = phi_prime(prime_p) * phi_prime(prime_q);
        let carmichaels = lcm(phi_prime(prime_p), phi_prime(prime_q));

        // 1 and 2 are never valid -- 2 < e < charmichaels(n)
        let mut encrypt_key = 2;
        while encrypt_key < carmichaels {
            if gcd(encrypt_key, carmichaels) == 1 {
                break;
            };
            encrypt_key += 1;
        }
        let decrypt_key = mod_inverse(encrypt_key, carmichaels);
        println!("{carmichaels}, {encrypt_key}, {decrypt_key}");

        Self {
            n,
            encrypt_key,
            decrypt_key,
        }
    }

    pub fn encrypt(&self, message: u64) -> u64 {
        pow_mod(message, self.encrypt_key, self.n)
    }
    pub fn decrypt(&self, cipher_text: u64) -> u64 {
        pow_mod(cipher_text, self.decrypt_key, self.n)
    }
}

fn carmichael(n: u64) -> u32 {
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

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

fn phi(n: u64) -> u64 {
    let mut count = 1;
    for i in 1..(n - 1) {
        if gcd(i, n) == 1 {
            count += 1
        }
    }
    count
}

fn mod_inverse(element: u64, modulus: u64) -> u64 {
    let mut answer = 1;
    while (element * answer) % modulus != 1 {
        answer += 1;
    }
    answer
}

fn pow_mod(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut total = base;
    for i in 1..exp {
        total = (total * base) % modulus
    }
    return total;
}

#[inline(always)]
fn phi_prime(prime: u64) -> u64 {
    prime - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rsa() {
        let cipher = Rsa::new(3, 7);
        let message = 12;
        let cipher_text = cipher.encrypt(message);
        let decrypted_message = cipher.decrypt(cipher_text);
        println!("{message}, {cipher_text}, {decrypted_message}");
        assert_eq!(message, decrypted_message);
    }
    #[test]
    fn test_bigger_rsa() {
        let cipher = Rsa::new(61, 53);
        let message = 65;
        let cipher_text = cipher.encrypt(message);
        let decrypted_message = cipher.decrypt(cipher_text);
        println!("{message}, {cipher_text}, {decrypted_message}");
        assert_eq!(message, decrypted_message);
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

    #[test]
    fn test_powmod() {
        assert_eq!(pow_mod(2, 4, 5), 1);
        assert_eq!(pow_mod(4, 16, 23), 12);
        assert_eq!(pow_mod(69, 69, 43), 8);
    }
}
