use thiserror::Error;

pub struct Des {
    pub key: u64,
    pub subkeys: [u64; 16],
    pub subkeys_computed: bool,
}

impl Des {
    const IP_TABLE: [u8; 64] = [
        58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14,
        6, 64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11,
        3, 61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
    ];

    const PC1_TABLE: [u8; 56] = [
        57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3,
        60, 52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45,
        37, 29, 21, 13, 5, 28, 20, 12, 4,
    ];

    const PC2_TABLE: [u8; 48] = [
        14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41,
        52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
    ];

    const FP_TABLE: [u8; 64] = [
        40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62,
        30, 37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19,
        59, 27, 34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
    ];

    const EXPANSION_TABLE: [u8; 48] = [
        32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17,
        18, 19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1,
    ];

    // Define the 8 S-Boxes as 4x16 tables:
    const S1: [[u8; 16]; 4] = [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
    ];

    const S2: [[u8; 16]; 4] = [
        [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
        [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
        [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
    ];

    const S3: [[u8; 16]; 4] = [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
    ];

    const S4: [[u8; 16]; 4] = [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
        [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
        [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
        [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
    ];

    const S5: [[u8; 16]; 4] = [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
    ];

    const S6: [[u8; 16]; 4] = [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
        [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
        [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
        [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
    ];

    const S7: [[u8; 16]; 4] = [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
    ];

    const S8: [[u8; 16]; 4] = [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
        [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
    ];

    // Group the S-Boxes into a static array:
    const S_BOXES: [[[u8; 16]; 4]; 8] = [
        Self::S1,
        Self::S2,
        Self::S3,
        Self::S4,
        Self::S5,
        Self::S6,
        Self::S7,
        Self::S8,
    ];

    const P_BOX: [u8; 32] = [
        16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9,
        19, 13, 30, 6, 22, 11, 4, 25,
    ];

    pub fn new(key: u64) -> Self {
        Self {
            key,
            subkeys: [0; 16],
            subkeys_computed: false,
        }
    }

    /// Permute the data using the table
    /// table: the permutation table
    /// input_key_length: the length of the input key in bits. If you want permute a 64-bit key, use 64.
    /// data: the data to permute
    ///
    /// Des typically numbers bits from left to right (1..64) (little endian):
    /// Bit 63 is the leftmost (most significant) bit in a u64.
    /// Bit 0 is the rightmost (least significant) bit.
    ///
    /// In Rust, we number bits from right to left (0..63) (big endian):
    /// Bit 0 is the rightmost (least significant) bit.
    /// Bit 63 is the leftmost (most significant) bit.
    pub fn permute(&self, table: &[u8], input_key_length: u8, data: u64) -> u64 {
        let mut output = 0u64;
        for i in table.iter() {
            // In the tables (left to right, 1-based):
            // bit 1 is the left most bit (MSB)
            // bit 64 is the right most bit (LSB)
            // In Rust (right to left, 0-based):
            // bit 0 is the right most bit (LSB)
            // bit 63 is the left most bit (MSB)
            // so the left most bit from the table goes to bit index 63
            let input_bit_position = input_key_length - i;
            // extract the bit from the data
            let bit = (data >> input_bit_position) & 1;
            // shift output by 1 to make space for the bit
            output <<= 1;
            // set the bit in the output
            output |= bit;
        }
        output
    }

    pub fn key_schedule(&mut self, key: u64) -> [u64; 16] {
        // Permutation Choice 1 -> 56 bits
        let permuted_key = self.perm_choice_1(key);
        // split the permuted key into two halves
        let (c0, d0) = self.split_key(permuted_key);
        let mut c = c0;
        let mut d = d0;
        // Left rotation
        for round in 0..16 {
            let shift = match round {
                0 | 1 | 8 | 15 => 1,
                _ => 2,
            };

            c = self.left_rotation(c, shift);
            d = self.left_rotation(d, shift);
            let key = ((c as u64) << 28) | (d as u64);

            // Permutation Choice 2
            self.subkeys[round] = self.perm_choice_2(key);
        }

        self.subkeys_computed = true;
        self.subkeys
    }

    pub fn split_key(&self, key: u64) -> (u32, u32) {
        // upper 28 bits
        let c0 = key >> 28;
        // lower 28 bits
        let d0 = key & ((1 << 28) - 1);
        (c0 as u32, d0 as u32)
    }

    /// This is a simple implementation of initial permutation
    /// TODO: use delta swap
    pub fn initial_permutation(&self, plain_text: u64) -> u64 {
        self.permute(&Self::IP_TABLE, 64, plain_text)
    }

    /// Reduce 64-bit key to 56-bit key by ignore every 8th bit
    /// DES typically numbers bits from left to right (1..64) (little endian)
    /// Bit 63 is the leftmost (most significant) bit in a u64.
    /// Bit 0 is the rightmost (least significant) bit.
    ///
    ///  <--- bit 63..bit 56                             bit 0 -->
    ///  +----------+------------------------------+-------------+
    ///  |8 bits 0  |               56 bits of data              |   
    ///  +----------+------------------------------+-------------+
    ///  0b0000_0000_XXXXXXXXXXXXXXXXXXXXXXXXXXXX_..._XXXXXXXXXX
    ///  ^^^^^^^^^^                            ^^^^^^^^^^^^^^^^^^
    ///         8 bits zero                  56 bits
    pub fn perm_choice_1(&mut self, key: u64) -> u64 {
        self.permute(&Self::PC1_TABLE, 64, key)
    }

    /// Reduce 56-bit key to 48-bit key by ignore every 8th bit
    pub fn perm_choice_2(&mut self, key: u64) -> u64 {
        self.permute(&Self::PC2_TABLE, 56, key)
    }

    pub fn final_permutation(&mut self, key: u64) -> u64 {
        self.permute(&Self::FP_TABLE, 64, key)
    }

    pub fn left_rotation(&mut self, key: u32, shift: u8) -> u32 {
        // shift all bits to the left (this might overflow)
        let shifted_left_bits = key << shift;

        // get the overflow bits
        let overflow_bits = key >> (28 - shift);

        // combine the shifted bits with the overflow bits
        let result = shifted_left_bits | overflow_bits;

        // mask the result to 28 bits
        let mask_28 = (1 << 28) - 1;
        result & mask_28
    }

    pub fn encrypt(&mut self, plain_text: u64) -> u64 {
        // initial permutation -> 64 bits
        let permuted_text = self.initial_permutation(plain_text);
        // spit the permuted data into two halves
        // left half (32 bits)
        let l0 = (permuted_text >> 32) as u32;
        // right half (32 bits)
        let r0 = (permuted_text & ((1 << 32) - 1)) as u32;
        // key schedule 16 Feistel Rounds
        let subkeys = if self.subkeys_computed {
            self.subkeys
        } else {
            self.key_schedule(self.key)
        };

        let (l16, r16) = self.feistel_rounds(l0, r0, &subkeys);
        // Final permutation
        // Combine (r16, l16) into a u64
        // Classic approach puts r16 on the left and l16 on the right
        let pre_final_permutation = (r16 as u64) << 32 | l16 as u64;
        self.final_permutation(pre_final_permutation)
    }

    pub fn feistel_round(&self, mut l: u32, mut r: u32, subkey: u64) -> (u32, u32) {
        // expand right half to 48 bits
        let expanded_right_half = self.expand_right_half(r);
        // round key mixing (xor with subkey)
        let mixed_right_half = expanded_right_half ^ subkey;
        // s-box substitution
        let s_box_output = self.s_box_substitution(mixed_right_half);
        // p-box permutation
        let p_box_output = self.p_box_substitution(s_box_output);
        // xor with left half
        let xored_output = p_box_output ^ l;
        // swap left and right halves
        l = r;
        r = xored_output;

        (l, r)
    }

    pub fn feistel_rounds(&self, mut l: u32, mut r: u32, subkeys: &[u64; 16]) -> (u32, u32) {
        for round in 0..16 {
            let subkey = subkeys[round];
            (l, r) = self.feistel_round(l, r, subkey);
        }

        (l, r)
    }

    pub fn feistel_rounds_rev(&self, mut l: u32, mut r: u32, subkeys: &[u64; 16]) -> (u32, u32) {
        for round in (0..16).rev() {
            let subkey = subkeys[round];
            (l, r) = self.feistel_round(l, r, subkey);
        }

        (l, r)
    }

    pub fn expand_right_half(&self, r: u32) -> u64 {
        self.permute(&Self::EXPANSION_TABLE, 32, r as u64)
    }

    /// Turn 6-bit chunk into 4-bit chunk
    /// chunk: 6-bit b5b4b3b2b1b0
    pub fn s_box_lookup(&self, s_box: &[[u8; 16]; 4], chunk: u64) -> u64 {
        // isolate b5 and move it to b0
        let top_bit = (chunk & 0b100000) >> 4;
        // isolate b0
        let bottom_bit = chunk & 0b000001;
        // row = top bit + bottom bit
        let row = top_bit | bottom_bit;
        // column = middle 4 bits. Isolate b4b3b2b1 then shift them down
        let column = (chunk & 0b011110) >> 1;
        // lookup the value in the s-box
        let value = s_box[row as usize][column as usize];
        // return the value as a u64
        value.into()
    }

    pub fn s_box_substitution(&self, chunk: u64) -> u32 {
        let mut output = 0u32;
        // in DES sense, we process chunks from left to right
        // but in Rust, we process chunks from right to left
        // i = 0 chunk: b47..b42, input_shift = 6 * (7-0) = 42
        // i = 1 chunk: b41..b36, input_shift = 6 * (7-1) = 36
        // i = 2 chunk: b35..b30, input_shift = 6 * (7-2) = 30
        // i = 3 chunk: b29..b24, input_shift = 6 * (7-3) = 24
        // i = 4 chunk: b23..b18, input_shift = 6 * (7-4) = 18
        // i = 5 chunk: b17..b12, input_shift = 6 * (7-5) = 12
        // i = 6 chunk: b11..b6, input_shift = 6 * (7-6) = 6
        // i = 7 chunk: b5..b0, input_shift = 6 * (7-7) = 0

        for i in 0..8 {
            let input_shift = 6 * (7 - i);
            // isolate the chunk with 6 bits
            let chunk = (chunk >> input_shift) & 0b111111;
            // lookup the value in the s-box to get a 4-bits value
            let value = self.s_box_lookup(&Self::S_BOXES[i], chunk);
            // place the value in the output
            // i = 0, output_shift = 4 * (7 - 0) = 28, output_chunk: b27..b24
            // i = 1, output_shift = 4 * (7 - 1) = 24, output_chunk: b23..b20
            // i = 2, output_shift = 4 * (7 - 2) = 20, output_chunk: b19..b16
            // i = 3, output_shift = 4 * (7 - 3) = 16, output_chunk: b15..b12
            // i = 4, output_shift = 4 * (7 - 4) = 12, output_chunk: b11..b8
            // i = 5, output_shift = 4 * (7 - 5) = 8, output_chunk: b7..b4
            // i = 6, output_shift = 4 * (7 - 6) = 4, output_chunk: b3..b0
            let output_shift = 4 * (7 - i);
            output |= (value as u32) << output_shift;
        }
        output
    }

    /// Introduce diffusion to the s-box output
    pub fn p_box_substitution(&self, chunk: u32) -> u32 {
        self.permute(&Self::P_BOX, 32, chunk as u64) as u32
    }

    pub fn decrypt(&mut self, cipher_text: u64) -> u64 {
        // initial permutation -> 64 bits
        let permuted_text = self.initial_permutation(cipher_text);
        // spit the permuted data into two halves
        // left half (32 bits)
        let l0 = (permuted_text >> 32) as u32;
        // right half (32 bits)
        let r0 = (permuted_text & ((1 << 32) - 1)) as u32;
        // key schedule 16 Feistel Rounds
        let subkeys = self.key_schedule(self.key);
        let (l16, r16) = self.feistel_rounds_rev(l0, r0, &subkeys);
        // Final permutation
        // Combine (r16, l16) into a u64
        // Classic approach puts r16 on the left and l16 on the right
        let pre_final_permutation = (r16 as u64) << 32 | l16 as u64;
        self.final_permutation(pre_final_permutation)
    }
}

#[derive(Error, Debug)]
pub enum DesError {}

#[cfg(test)]
mod tests {
    use super::Des;
    use crate::common::hex::Hex;

    #[test]
    fn test_ip() {
        let key = Hex::new("hi").unwrap();
        let des = Des::new(u64::from(key));
        let plain_text = Hex::new("hello").unwrap();
        let ip_output = des.initial_permutation(u64::from(plain_text));
        assert_eq!(ip_output, 17870547822688397440u64);
    }

    #[test]
    fn test_pc1() {
        let mut des = Des::new(26729u64);
        let pc1_output = des.perm_choice_1(26729u64);
        assert_eq!(pc1_output, 211930866256896u64);
    }

    #[test]
    fn test_pc2() {
        let mut des = Des::new(26729u64);
        let pc2_output = des.perm_choice_2(26729u64);
        assert_eq!(pc2_output, 49496u64);
    }

    #[test]
    fn test_left_rotation() {
        let mut des = Des::new(26729u64);
        let left_rotation_output = des.left_rotation(26729u32, 1);
        assert_eq!(left_rotation_output, 53458u32);
    }

    #[test]
    fn test_encrypt() {
        let mut des = Des::new(26729u64);
        let plain_text = Hex::new("hello").unwrap();
        let encrypted_text = des.encrypt(u64::from(plain_text));
        assert_eq!(format!("{:X}", encrypted_text), "A74BCE81F3679AEF");

        let plain_text = Hex::new("world").unwrap();
        let encrypted_text = des.encrypt(u64::from(plain_text));
        assert_eq!(format!("{:X}", encrypted_text), "BAFFC2305F333606");
    }

    #[test]
    fn test_decrypt() {
        let mut des = Des::new(26729u64);
        let cipher_text = Hex::new("A74BCE81F3679AEF").unwrap();
        let decrypted_text = des.decrypt(u64::from(cipher_text));

        let plain_text = Hex::new("hello").unwrap();
        assert_eq!(format!("{:X}", decrypted_text), format!("{}", plain_text));
    }
}
