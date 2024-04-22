use crate::math;

use math::*;

struct Rsa {
    n: u128,
    encrypt_key: u128,
    decrypt_key: u128,
}

impl Rsa {
    pub fn new(prime_p: u128, prime_q: u128) -> Self {
        let n = prime_p * prime_q;
        // let phi = phi_prime(prime_p) * phi_prime(prime_q);
        let carmichaels = u128::lcm(prime_p.phi_prime(), prime_q.phi_prime());

        // 1 and 2 are never valid -- 2 < e < charmichaels(n)
        let mut encrypt_key = 2;
        while encrypt_key < carmichaels {
            if u128::gcd(encrypt_key, carmichaels) == 1 {
                break;
            };
            encrypt_key += 1;
        }
        let decrypt_key = encrypt_key.mod_inverse(carmichaels);
        println!("{carmichaels}, {encrypt_key}, {decrypt_key}");

        Self {
            n,
            encrypt_key,
            decrypt_key,
        }
    }

    pub fn encrypt(&self, message: u128) -> u128 {
        message.pow_mod(self.encrypt_key, self.n)
    }
    pub fn decrypt(&self, cipher_text: u128) -> u128 {
        cipher_text.pow_mod(self.decrypt_key, self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let cipher = Rsa::new(3, 7);
        let message = 12;
        let cipher_text = cipher.encrypt(message);
        let decrypted_message = cipher.decrypt(cipher_text);
        assert_eq!(message, decrypted_message);
    }
    #[test]
    fn test_bigger_rsa() {
        let cipher = Rsa::new(61, 53);
        let message = 65;
        let cipher_text = cipher.encrypt(message);
        let decrypted_message = cipher.decrypt(cipher_text);
        assert_eq!(message, decrypted_message);
    }
}
