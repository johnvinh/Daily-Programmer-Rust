pub fn solve(name: String, age: String, username: String) -> &'static str {
    "failing test"
}

#[cfg(test)]
mod tests {

    #[test]
    fn final_output() {
        let name = String::from("Matthew");
        let age = String::from("17");
        let username = String::from("test-username");
        assert_eq!(solve(name, age, username), )
    }
}