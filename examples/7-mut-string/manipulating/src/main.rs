use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Commenting out the examples from the demo

    // // Use slicing to get the first three characters of the sentence
    // println!("{}", &sentence[0..=4]);

    // // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", description);

    // // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // // Split and collect into a vector
    // //let words: Vec<&str> = sentence.split_whitespace().collect();
    // let words = sentence.split(' ').collect::<Vec<&str>>();
    // //println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // // println!("{}", reversed);

    // Solutions in the lab
    
    fn longest_word(str: &String) -> String {
        let words = str.split(' ').collect::<Vec<&str>>();

        // // Option 1: Using max_by_key
        // if let Some(longest) = words.iter().max_by_key(|word| word.len()) {
        //     println!("Longest word: {}", longest);
        // }

        // Option 2: Manual loop
        let mut longest = "";
        for &word in &words {
            if word.len() > longest.len() {
                longest = word;
            }
        }
        println!("Longest word: {}", longest);
        longest.to_string()
    }
    println!("Longest word in sentence: {}", longest_word(&sentence));
    
    // HashMap for each vowel count
    let mut vowel_counts = HashMap::new();
    vowel_counts.insert('a', 0);
    vowel_counts.insert('e', 0);
    vowel_counts.insert('i', 0);
    vowel_counts.insert('o', 0);
    vowel_counts.insert('u', 0);
    
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                *vowel_counts.get_mut(&c).unwrap() += 1;
            }
            _ => continue,
        }
    }
    
    println!("Vowel counts: {:?}", vowel_counts);
}
