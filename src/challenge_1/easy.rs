use std::io;

pub fn get_name() -> Result<String, io::Error> {
    println!("Name:");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_n) => Ok(name.trim().to_string()),
        Err(e) => Err(e),
    }
}

pub fn get_age() -> Result<String, io::Error> {
    println!("Age:");
    let mut age = String::new();
    match io::stdin().read_line(&mut age) {
        Ok(_n) => Ok(age.trim().to_string()),
        Err(e) => Err(e),
    }
}

pub fn get_username() -> Result<String, io::Error> {
    println!("Username:");
    let mut username = String::new();
    match io::stdin().read_line(&mut username) {
        Ok(_n) => Ok(username.trim().to_string()),
        Err(e) => Err(e)
    }
}

pub fn solve(name: String, age: String, username: String) -> String {
    format!("your name is {}, you are {} years old, and your username is {}", name, age, username)
}

pub fn run() -> Result<(), io::Error> {
    let name: String = get_name()?;
    let age: String = get_age()?;
    let username: String = get_username()?;
    println!("{}", solve(name, age, username));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_output() {
        let name = String::from("Matthew");
        let age = String::from("17");
        let username = String::from("test-username");
        assert_eq!(solve(name, age, username),
                   String::from("your name is Matthew, you are 17 years old, and your username is test-username"));
    }
}