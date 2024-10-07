fn most_frequent_word(text: &str) -> (String, usize) {
    let binding = text.to_lowercase();
    let words = binding.split_whitespace().collect::<Vec<_>>();
    let mut word_counts: Vec<(&str, usize)> = Vec::new();

    for word in &words {
        let mut found = false;
        for (_, (count_word, count)) in word_counts.iter_mut().enumerate() {
            if *count_word == *word {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            word_counts.push((word, 1));
        }
    }

    let mut max_count = 0;
    let mut max_word = "";
    for (word, count) in &word_counts {
        if *count > max_count {
            max_count = *count;
            max_word = word;
        }
    }
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
