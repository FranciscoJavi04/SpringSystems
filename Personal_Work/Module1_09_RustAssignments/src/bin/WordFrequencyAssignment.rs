fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word: &str = "";
    let mut max_count: usize = 0;

    for i in 0..words.len() {
        let current_word = words[i];
        let mut count = 0;

        for j in 0..words.len() {
            if words[j] == current_word {
                count += 1;
            }
        }

        if count > max_count {
            update_max(&mut max_word, &mut max_count, current_word, count);
        }
    }

    (max_word.to_string(), max_count)
}

fn update_max<'a>(
    max_word: &mut &'a str,
    max_count: &mut usize,
    new_word: &'a str,
    new_count: usize,
) {
    *max_word = new_word;
    *max_count = new_count;
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}