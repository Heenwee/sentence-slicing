pub fn slice_string(s: &String) -> [&str; 10] {
    let bytes = s.as_bytes();
    let mut words: [&str; 10] = ["","","","","","","","","",""];

    let mut n: usize = 0;
    let mut spot: usize = 0;

    let mut letters: bool = true;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b'.' || item == b',' {
            if letters {
                words[n] = &s[spot..i];
                n += 1;
            }
            letters = false;
        }
        if !letters && !(item == b' ' || item == b'.' || item == b',') {
            letters = true;
            spot = i;
        }
    }
    words[n] = &s[spot..];

    words
}

pub fn check_for_word(sentence: String, word: &str) -> bool {
    let words: [&str; 10] = slice_string(&sentence);
    let mut found_word: bool = false;

    for i in words {
        if i == word {
            found_word = true;
            break;
        }
    }

    found_word
}