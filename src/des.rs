#[derive(Debug, PartialEq)]
struct Sdes {
    key1: u8,
    key2: u8,
}

impl Sdes {
    fn new(mut key: u16) -> Self {
        assert!(key < 2048);

        key = permutation_10(key);

        key = ls_1(key);
        let key1 = permutation_8(key as u8);

        key = ls_2(key);
        let key2 = permutation_8(key as u8);

        Self { key1, key2 }
    }

    fn encrypt(&self, bits: u8) -> u8 {
        calculate(bits, self.key1, self.key2)
    }

    fn decyrpt(&self, bits: u8) -> u8 {
        calculate(bits, self.key2, self.key1)
    }
}

fn calculate(mut bits: u8, first_key: u8, second_key: u8) -> u8 {
    bits = permutation_ip(bits);

    bits = step(first_key, bits);
    // reverse the nibbles
    bits = (bits >> 4) | ((bits & 0b1111) << 4);
    bits = step(second_key, bits);

    bits = permutation_inverse_ip(bits);
    bits
}

fn step(key: u8, mut bits: u8) -> u8 {
    let left = bits >> 4;
    let right = bits & 0b1111;

    bits = permutation_ep(right);
    bits ^= key;
    bits = sboxes(bits);
    bits = permutation_4(bits);
    bits ^= left;
    (bits << 4) | right
}

fn sbox_index(bits: u8) -> usize {
    let row = (bits & 0b0001) | ((bits & 0b1000) >> 2);
    let col = (bits & 0b0110) >> 1;
    ((row * 4) + col) as usize
}

fn sboxes(bits: u8) -> u8 {
    const S0: [u8; 16] = [1, 0, 3, 2, 3, 2, 1, 0, 0, 2, 1, 3, 3, 1, 3, 2];
    const S1: [u8; 16] = [0, 1, 2, 3, 2, 0, 1, 3, 3, 0, 1, 0, 2, 1, 0, 3];

    let right = bits & 0b1111;
    let left = bits >> 4;

    let i0 = sbox_index(left);
    let i1 = sbox_index(right);
    (S0[i0] << 2) | S1[i1]
}

fn ls_1(key: u16) -> u16 {
    let mut left = key >> 5;
    let mut right = key & 0b11111;
    left = shl_u5(left);
    right = shl_u5(right);
    (left << 5) | right
}

fn ls_2(key: u16) -> u16 {
    let mut left = key >> 5;
    let mut right = key & 0b11111;
    left = shl_u5(left);
    left = shl_u5(left);
    right = shl_u5(right);
    right = shl_u5(right);
    (left << 5) | right
}

fn shl_u5(bits: u16) -> u16 {
    let overflow_bit = (bits & 0b10000) >> 4;
    (bits << 1) & 0b11111 | overflow_bit
}

fn permutation_4(bits: u8) -> u8 {
    (bits & 0b1000) >> 3
        | (bits & 0b0100) << 1
        | (bits & 0b0010)
        | (bits & 0b0001) << 2
}

fn permutation_ep(bits: u8) -> u8 {
    (bits & 0b0001) << 7
        | (bits & 0b1000) << 3
        | (bits & 0b0100) << 3
        | (bits & 0b0010) << 3
        | (bits & 0b0100) << 1
        | (bits & 0b0010) << 1
        | (bits & 0b0001) << 1
        | (bits & 0b1000) >> 3
}

fn permutation_inverse_ip(bits: u8) -> u8 {
    (bits & 0b10000000) >> 1
        | (bits & 0b01000000) >> 4
        | (bits & 0b00100000)
        | (bits & 0b00010000) << 3
        | (bits & 0b00001000) << 1
        | (bits & 0b00000100) >> 2
        | (bits & 0b00000010) << 2
        | (bits & 0b00000001) << 1
}

fn permutation_ip(bits: u8) -> u8 {
    (bits & 0b10000000) >> 3
        | (bits & 0b01000000) << 1
        | (bits & 0b00100000)
        | (bits & 0b00010000) >> 1
        | (bits & 0b00001000) >> 2
        | (bits & 0b00000100) << 4
        | (bits & 0b00000010) >> 1
        | (bits & 0b00000001) << 2
}

fn permutation_8(bits: u8) -> u8 {
    let bits = bits as u8;
    (bits & 0b10000000) >> 1
        | (bits & 0b01000000) >> 2
        | (bits & 0b00100000) >> 3
        | (bits & 0b00010000) << 3
        | (bits & 0b00001000) << 2
        | (bits & 0b00000100) << 1
        | (bits & 0b00000010) >> 1
        | (bits & 0b00000001) << 1
}

fn permutation_10(bits: u16) -> u16 {
    (bits & 0b1000000000) >> 6
        | (bits & 0b0100000000) >> 1
        | (bits & 0b0010000000) << 2
        | (bits & 0b0001000000) >> 1
        | (bits & 0b0000100000) << 3
        | (bits & 0b0000010000) >> 4
        | (bits & 0b0000001000) << 3
        | (bits & 0b0000000100) >> 1
        | (bits & 0b0000000010) << 1
        | (bits & 0b0000000001) << 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdes_keygen() {
        let key = 0b1010000010;
        let sdes = Sdes::new(key);

        assert_eq!(
            sdes,
            Sdes {
                key1: 0b10100100,
                key2: 0b01000011
            }
        )
    }

    #[test]
    fn test_sdes_encryption() {
        let key = 0b1010000010;
        let sdes = Sdes::new(key);
        let ciphertext = sdes.encrypt(0b10010111);

        assert_eq!(ciphertext, 0b00111000)
    }

    #[test]
    fn test_sdes_decryption1() {
        let key = 0b1010000010;
        let sdes = Sdes::new(key);
        let ciphertext = sdes.decyrpt(0b00111000);

        assert_eq!(ciphertext, 0b10010111)
    }

    #[test]
    fn test_sdes_decryption2() {
        let key = 0b1100101001;
        let sdes = Sdes::new(key);
        let data = sdes.decyrpt(0b00011001);

        assert_eq!(data, 0b10100110)
    }

    #[test]
    fn test_permutation10() {
        assert_eq!(permutation_10(0b1010000010), 0b1000001100)
    }
}
