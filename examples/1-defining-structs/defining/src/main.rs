
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    age: u8,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let person = Person {
        first_name: "Skyler".to_string(),
        last_name: "McMullen".to_string(),
        email: "skyler.d.mcmullen@gmail.com".to_string(),
        phone_number: "720-273-3207".to_string(),
        age: 27,
    };
    println!("person: {}", person.full_name());
}
