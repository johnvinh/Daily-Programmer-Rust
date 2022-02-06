use std::io;
use std::fmt;
use std::fmt::Formatter;

pub fn run(events: &mut Vec<Event>) {
    let mut input: String = String::new();
    let mut quit: bool = false;
    while !quit {
        println!("1. Add Event");
        println!("2. Edit Event");
        println!("3. List Events");
        println!("4. Quit");
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("There was an error reading your input. Please try again.");
            continue;
        }
        match input.trim() {
            "1" => {
                if let Ok(()) = add_event(events) {
                    println!("Event successfully added!");
                } else {
                    println!("Failed to add event.");
                }
            },
            "3" => {
                for event in &*events {
                    println!("{}", event);
                }
            },
            "4" => quit = true,
            _ => println!("Invalid option. Please try again.")
        }
        input.clear();
    }
}

pub struct Event {
    name: String,
    hour: i32,
}

impl Event {
    pub fn new(name: String, hour: i32) -> Event {
        Event {name, hour}
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Event Name: {}, Event Hour: {}", self.name, self.hour)
    }
}

fn add_event(events: &mut Vec<Event>) -> Result<(), &'static str> {
    let mut event_name: String = String::new();
    println!("Event Name:");
    if let Err(_e) = io::stdin().read_line(&mut event_name) {
        return Err("Failed to read name");
    }
    let mut event_hour: String = String::new();
    println!("Event Hour:");
    if let Err(_e) = io::stdin().read_line(&mut event_hour) {
        return Err("Failed to read hour");
    }
    // Make sure the hour is an integer
    let event_hour: i32 = match event_hour.trim().parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid hour"),
    };
    // Add the new event
    events.push(Event::new(event_name.trim().to_string(), event_hour));
    Ok(())
}