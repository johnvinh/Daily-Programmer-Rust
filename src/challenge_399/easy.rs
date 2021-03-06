use std::collections::HashMap;
use std::fs;
use std::io;

fn get_letter_values() -> HashMap<char, i32> {
    let mut letter_values: HashMap<char, i32> = HashMap::new();
    letter_values.insert('a', 1);
    letter_values.insert('b', 2);
    letter_values.insert('c', 3);
    letter_values.insert('d', 4);
    letter_values.insert('e', 5);
    letter_values.insert('f', 6);
    letter_values.insert('g', 7);
    letter_values.insert('h', 8);
    letter_values.insert('i', 9);
    letter_values.insert('j', 10);
    letter_values.insert('k', 11);
    letter_values.insert('l', 12);
    letter_values.insert('m', 13);
    letter_values.insert('n', 14);
    letter_values.insert('o', 15);
    letter_values.insert('p', 16);
    letter_values.insert('q', 17);
    letter_values.insert('r', 18);
    letter_values.insert('s', 19);
    letter_values.insert('t', 20);
    letter_values.insert('u', 21);
    letter_values.insert('v', 22);
    letter_values.insert('w', 23);
    letter_values.insert('x', 24);
    letter_values.insert('y', 25);
    letter_values.insert('z', 26);
    letter_values
}

pub fn get_challenge_words() -> Result<Vec<String>, io::Error> {
    let words = fs::read_to_string("399_easy.txt")?;
    let words = words.split("\n").collect::<Vec<&str>>();
    let mut out = Vec::new();
    for word in words {
        out.push(String::from(word));
    }
    Ok(out)
}

pub fn letter_sum(word: &str) -> i32 {
    let letter_values = get_letter_values();
    let mut sum: i32 = 0;
    for character in word.chars() {
        sum += match letter_values.get(&character) {
            Some(&value) => value,
            None => 0,
        };
    }
    sum
}

pub mod bonus {
    use super::*;

    pub fn solve() {
        let words = get_challenge_words().unwrap();

        // Question 1
        let sum_319 = || {
            for word in &words {
                let sum = letter_sum(word);
                if sum == 319 {
                    return word.clone();
                }
            }
            String::from("")
        };
        // Question 2
        let odd_sum = || {
            let mut count = 0;
            for word in &words {
                let sum = letter_sum(word);
                if sum % 2 != 0 {
                    count += 1;
                }
            }
            count
        };

        println!("The word with a letter sum of 319 is: {}", sum_319());
        println!("There are {} words with an odd letter sum.", odd_sum());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_sum() {
        // Test cases provided by the problem description
        assert_eq!(letter_sum(""), 0);
        assert_eq!(letter_sum("a"), 1);
        assert_eq!(letter_sum("z"), 26);
        assert_eq!(letter_sum("cab"), 6);
        assert_eq!(letter_sum("excellent"), 100);
        assert_eq!(letter_sum("microspectrophotometries"), 317);
    }
}