mod sentence_slicer;
use sentence_slicer::check_for_word;

fn main() {
    let sentence: String = String::from("This is, a sentence.");
    let word: &str = "is";

    println!("{}", sentence);
    //println!("{:?}", words);
    
    match check_for_word(sentence, word) {
        false =>    println!("Sentence does not contain \"{}\"", word),
        true =>     println!("\"{}\" found", word)
    };
}