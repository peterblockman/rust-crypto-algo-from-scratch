use crate::{
    math::modular_arithmetic::mod_inverse::mod_inverse,
    symmetric_encryption::utils::{alphabet_size_i32, get_char_at_index, get_char_index, ALPHABET},
};

/// Affine Cipher Encryption
pub fn encrypt(text: &str, a: i32, b: i32) -> Result<String, String> {
    text.chars().try_fold(String::new(), |mut acc, c| {
        let Some(index) = ALPHABET.find(c) else {
            return Err("Invalid character".to_string());
        };

        // new_index = (a * index as i32 + b) % alphabet_size_i32();
        let new_index = (a * index as i32 + b).rem_euclid(alphabet_size_i32());

        let new_char = get_char_at_index(new_index)?;

        acc.push(new_char);

        Ok(acc)
    })
}

/// Affine Cipher Decryption
pub fn decrypt(cipher: &str, a: i32, b: i32) -> Result<String, String> {
    let a_inverse = mod_inverse(a as i32, alphabet_size_i32())?;

    cipher.chars().try_fold(String::new(), |mut acc, c| {
        let index = get_char_index(c)?;

        let new_index = (a_inverse * (index as i32 - b)).rem_euclid(alphabet_size_i32());

        let new_char = get_char_at_index(new_index)?;

        acc.push(new_char);

        Ok(acc)
    })
}

#[test]
fn test_affine() {
    let text = "hello";
    let cipher = "ctoox";
    let a = 3;
    let b = 7;
    let encrypted_text = encrypt(text, a, b).unwrap();
    assert_eq!(cipher, &encrypted_text);

    let decrypted_text = decrypt(cipher, a, b).unwrap();
    assert_eq!(text, &decrypted_text);
}
