struct Event {
    name: String,
    hour: i32,
}

impl Event {
    pub fn new(name: String, hour: i32) -> Event {
        Event {name, hour}
    }
}