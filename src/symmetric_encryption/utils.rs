pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn alphabet_size_i32() -> i32 {
    ALPHABET.len() as i32
}

pub fn get_char_index(c: char) -> Result<i32, String> {
    let Some(index) = ALPHABET.find(c) else {
        return Err("Invalid character".to_string());
    };

    Ok(index as i32)
}

pub fn get_char_at_index(index: i32) -> Result<char, String> {
    ALPHABET
        .chars()
        .nth(index as usize)
        .ok_or("Invalid character".to_string())
}
