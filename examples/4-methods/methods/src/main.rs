#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn from_email(email: String) -> Self {
        let (username, _domain) = email.split_once('@')
            .unwrap_or(("", ""));
        Self {
            username: String::from(username),
            email,
            uri: String::from(""),
            active: true,
        }
        }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn update_uri(&mut self, uri: String) {
        self.uri = uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut another_user: User = User::from_email(
        String::from("skyler.mcmullen@gmail.com"));
    println!("{:?}", another_user);
    another_user.update_uri(String::from("https://sdmcmullen.com"));
    println!("{:?}", another_user)
}
