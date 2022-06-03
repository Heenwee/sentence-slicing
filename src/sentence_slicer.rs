pub fn slice_string(s: &String) -> Vec<&str> {
    let bytes = s.as_bytes();
    let mut words: Vec<&str> = Vec::new();

    let mut spot: usize = 0;

    let mut letters: bool = true;

    for (i, &item) in bytes.iter().enumerate() {
        if illegal_character(item) {
            if letters {
                words.push(&s[spot..i]);
            }
            letters = false;
        }
        if !letters && !(illegal_character(item)) {
            letters = true;
            spot = i;
        }
    }

    words
}

fn illegal_character(item: u8) -> bool {
    item == b' ' || item == b'.' || item == b',' || item == b'"' || item == b'/'
}

pub fn check_for_word(sentence: String, word: &str) -> bool {
    let words: Vec<&str> = slice_string(&sentence);
    let mut found_word: bool = false;

    for i in &words {
        if i.to_lowercase() == word {
            found_word = true;
            break;
        }
    }

    found_word
}