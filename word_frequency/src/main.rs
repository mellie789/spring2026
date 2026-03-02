/*
Create a program that:

Takes a string of text as input
Splits the text into words (space as separator) // text.split_whitespace().collect();
Counts the frequency of each word
Returns the word with the highest frequency and its count
Requirements:

Use mutable references where appropriate
Avoid using HashMaps or complex data structures
*/

fn most_frequent_word(text: &str) -> (String, usize) {
    //Split the text into words and collect into a vector
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return ("".to_string(), 0); // Return empty string and zero count if no words
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for i in 0..words.len(){
        let mut count = 0;

        for j in 0..words.len(){
            if words[i] == words[j]{
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = words[i].to_string();
        }
    }
    
    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}