struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: text.to_string(),
        }
    }

    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

use std::io;

fn main() {
    println!("Please enter some text:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let counter = WordCounter::new(&input_text);
    let word_count = counter.count_words();
    println!("Word count: {}", word_count);
}
