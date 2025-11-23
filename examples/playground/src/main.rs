use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    println!("=== Creating HashMaps ===");
    
    // Method 1: Create empty and insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
    
    // Method 2: From vectors using collect
    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_scores = vec![25, 30];
    let team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Team scores: {:?}", team_scores);
    
    // Accessing values
    println!("\n=== Accessing Values ===");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Team not found"),
    }
    
    // Using copied().unwrap_or() for default values
    let blue_score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    let orange_score = scores.get(&String::from("Orange")).copied().unwrap_or(0);
    println!("Blue: {}, Orange: {}", blue_score, orange_score);
    
    // Iterating over HashMap
    println!("\n=== Iterating ===");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    println!("\n=== Updating Values ===");
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("After update: {:?}", scores);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50);  // Won't insert
    scores.entry(String::from("Red")).or_insert(50);   // Will insert
    println!("After entry/or_insert: {:?}", scores);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;  // Dereference and increment
    }
    println!("\n=== Word Count Example ===");
    println!("Word frequencies: {:?}", word_count);
    
    // Ownership considerations
    println!("\n=== Ownership ===");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now owned by the map
    // println!("{}", field_name); // This would error!
    
    // With references (using lifetimes)
    let key = "name";
    let value = "Alice";
    let mut ref_map = HashMap::new();
    ref_map.insert(key, value);
    println!("Can still use: {} = {}", key, value); // Works with &str
    
    // Common methods
    println!("\n=== Common Methods ===");
    println!("Contains 'Red': {}", scores.contains_key(&String::from("Red")));
    println!("Length: {}", scores.len());
    
    scores.remove(&String::from("Yellow"));
    println!("After removing Yellow: {:?}", scores);
    
    // Custom types as keys
    println!("\n=== Custom Types ===");
    
    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let mut person_scores = HashMap::new();
    person_scores.insert(
        Person { name: String::from("Alice"), age: 30 },
        95,
    );
    person_scores.insert(
        Person { name: String::from("Bob"), age: 25 },
        87,
    );
    
    for (person, score) in &person_scores {
        println!("{:?} scored {}", person, score);
    }
}
