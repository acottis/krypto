pub struct DiffieHellman {
    prime: u128,
    prime_root: u32,
}

impl DiffieHellman {
    pub fn new(prime: u128, prime_root: u32) -> Self {
        Self { prime, prime_root }
    }

    pub fn generate_exchange_key(&self, key: u32) -> u32 {
        let prime_root = self.prime_root as u128;
        (prime_root
            .checked_pow(key)
            .expect("Overflow on Generating Exchange Key")
            % self.prime)
            .try_into()
            .unwrap()
    }

    pub fn shared_secret(&self, key: u32, exchange_key: u32) -> u64 {
        let exchange_key = exchange_key as u128;
        (exchange_key
            .checked_pow(key)
            .expect("Overflow on Generating Exchange Key")
            % self.prime)
            .try_into()
            .unwrap()
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
}
