


fn find_longest_word(mut text: String) -> (String, usize) {
    //Clean text from non-alphabetic symbols
    text = text.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
    //Split text into vector of strings(Words in text)
    let words = text.split(" ");
    let mut max_word_length = 0;
    let mut longest_word: String = String::from("");
    //Run over all Words and check for the longest one (only the )
    for word in words{
        if max_word_length < word.len() {
            max_word_length = word.len();
            longest_word = word.to_string();
        };
    }
    return (longest_word, max_word_length);
}

fn main() {
    let text = String::from("This is an Experiment.");
    let result = find_longest_word(text);
    let longest_word = result.0;
    let length = result.1;
    println!("The longest word is '{longest_word}', the last word is {length} long");
}
