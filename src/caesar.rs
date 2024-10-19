const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn encrypt(text: &str, shift: u32) -> Result<String, String> {
    text.chars().try_fold(String::new(), |mut acc, c| {
        let is_uppercase = c.is_ascii_uppercase();

        let c = if is_uppercase {
            c.to_ascii_lowercase()
        } else {
            c
        };

        let Some(index) = ALPHABET.find(c) else {
            return Err("Invalid character".to_string());
        };
        // wrap around if index + shift is greater than ALPHABET.len()
        let new_index = (index + shift as usize) % ALPHABET.len();

        let new_char = ALPHABET
            .chars()
            .nth(new_index)
            .ok_or("Invalid character".to_string())?;

        acc.push(if is_uppercase {
            new_char.to_ascii_uppercase()
        } else {
            new_char
        });

        Ok(acc)
    })
}

pub fn decrypt(cipher: &str, shift: u32) -> Result<String, String> {
    // In a 26-letter alphabet, shifting backward by n is equivalent to shifting forward by 26 - n.
    encrypt(cipher, ALPHABET.len() as u32 - shift)
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

    let non_alphabetic_text = "Hello, World!";
    let err = encrypt(non_alphabetic_text, shift).unwrap_err();
    assert_eq!(&err, "Invalid character");
}
