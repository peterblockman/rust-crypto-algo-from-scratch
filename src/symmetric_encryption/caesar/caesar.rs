use crate::symmetric_encryption::utils::{alphabet_size_i32, get_char_at_index, get_char_index};

pub fn encrypt(text: &str, mut shift: i32) -> Result<String, String> {
    // handle negative shift
    shift = if shift < 0 {
        alphabet_size_i32() + shift
    } else {
        shift
    };

    text.chars().try_fold(String::new(), |mut acc, c| {
        let is_uppercase = c.is_ascii_uppercase();

        let c = if is_uppercase {
            c.to_ascii_lowercase()
        } else {
            c
        };

        let index = get_char_index(c)?;
        // wrap around if index + shift is greater than ALPHABET.len()
        // new_index = index + shift % alphabet_size_i32()
        // using rem_euclid instead of %, to ensure that the new_index is always within [0, n-1]
        let new_index = (index + shift).rem_euclid(alphabet_size_i32());

        let new_char = get_char_at_index(new_index)?;

        acc.push(if is_uppercase {
            new_char.to_ascii_uppercase()
        } else {
            new_char
        });

        Ok(acc)
    })
}

pub fn decrypt(cipher: &str, shift: i32) -> Result<String, String> {
    // In a 26-letter alphabet, shifting backward by n is equivalent to shifting forward by 26 - n.
    encrypt(cipher, alphabet_size_i32() - shift)
}

#[test]
fn caesar_test() {
    let text = "Hello";
    let cipher = "Rovvy";
    let shift = 10;
    let encrypted_text = encrypt(text, shift).unwrap();
    assert_eq!(&encrypted_text, cipher);
    let decrypted_text = decrypt(cipher, shift).unwrap();
    assert_eq!(&decrypted_text, text);

    // negative shift
    let shift = -1;
    let cipher = "Gdkkn";
    let encrypted_text = encrypt(text, shift).unwrap();
    let decrypted_text = decrypt(cipher, shift).unwrap();
    assert_eq!(&encrypted_text, cipher);
    assert_eq!(&decrypted_text, text);

    let non_alphabetic_text = "Hello, World!";
    let err = encrypt(non_alphabetic_text, shift).unwrap_err();
    assert_eq!(&err, "Invalid character");
}
