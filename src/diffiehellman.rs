use crate::math::*;

pub struct DiffieHellman {
    prime: u64,
    prime_root: u64,
}

impl DiffieHellman {
    pub fn new(prime: u64, prime_root: u64) -> Self {
        Self { prime, prime_root }
    }

    pub fn generate_exchange_key(&self, key: u64) -> u64 {
        self.prime_root.pow_mod(key, self.prime)
    }

    pub fn shared_secret(&self, key: u64, exchange_key: u64) -> u64 {
        exchange_key.pow_mod(key, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn diffie_hellman_exchange() {
        let dh = DiffieHellman::new(23, 5);

        let alice_key = 5;
        let bob_key = 4;

        let a = dh.generate_exchange_key(alice_key);
        let b = dh.generate_exchange_key(bob_key);

        let alice_shared_secret = dh.shared_secret(alice_key, b);
        let bob_shared_secret = dh.shared_secret(bob_key, a);
        assert!(alice_shared_secret == bob_shared_secret)
    }

    #[test]
    pub fn diffie_hellman_exchange_bigger() {
        let dh = DiffieHellman::new(239, 83);

        let alice_key = 5;
        let bob_key = 4;

        let a = dh.generate_exchange_key(alice_key);
        let b = dh.generate_exchange_key(bob_key);

        let alice_shared_secret = dh.shared_secret(alice_key, b);
        let bob_shared_secret = dh.shared_secret(bob_key, a);
        assert!(alice_shared_secret == bob_shared_secret)
    }
}
