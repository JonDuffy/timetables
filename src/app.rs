use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

struct Student {
    //vector of classes
    classes: Vec<i32>
}

impl Student {
    // create a new student
    fn new() -> Student {
        Student { classes: Vec::new()}
    }
    // add a class to a student
    fn addClass(&mut self, class: i32) {
        self.classes.push(class)
    }
}

struct Room {
    // room capacity
    capacity: Option<i32>,
    // vector of features
    features: Vec<i32>
}

impl Room {
    // create a new room
    fn new(capacity: i32) -> Room {
        Room {capacity: Some(capacity), features: Vec::new()}
    }

    fn add_feature(&mut self, feature: i32) {
        self.features.push(feature)
    }

    fn get_capacity(&self) -> i32 {
        self.capacity.unwrap()
    }
}

struct Feature {

}

impl Feature {
    fn new() -> Feature {
        Feature {}
    }
}

struct Event {
    //mustBeBefore: link to Event
    must_be_before: Vec<Box<Event>>,
    //must be after: link to event
    must_be_after: Vec<Box<Event>>,
    //vector of features
    features: Vec<Box<Feature>>,
    //vector of compatible timeslots
    timeslots: Vec<i32>
}

impl Event {
    // create new Event
    fn new() -> Event {
        Event {must_be_before: Vec::new(), must_be_after: Vec::new(), features: Vec::new(), timeslots: Vec::new()}
    }

    fn set_must_be_before(&mut self, event: Box<Event>) {
        self.must_be_before.push(event)
    }

    fn set_must_be_after(&mut self, event: Box<Event>) {
        self.must_be_after.push(event)
    }

    fn add_feature(&mut self, feature: Box<Feature>) {
        self.features.push(feature)
    }

    fn add_timeslot(&mut self, timeslot: i32) {
        self.timeslots.push(timeslot)
    }
}


fn main () {
    let mut data = String::new();
    let mut f = File::open("data/ittc/comp-2007-2-1.tim.txt").unwrap();
    f.read_to_string(&mut data).unwrap();

    let mut data: Vec<i32> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let num_events = data.remove(0);
    let num_rooms = data.remove(0);
    let num_features = data.remove(0);
    let num_students = data.remove(0);

    println!("number of events = {}",num_events);
    println!("number of rooms = {}",num_rooms);
    println!("number of features = {}",num_features);
    println!("number of students = {}",num_students);

    //collections of each type

    let mut rooms: Box<Vec<Room>> = Box::new(vec![]);
    let mut events: Box<Vec<Event>> = Box::new(vec![]);
    let mut students: Box<Vec<Student>> = Box::new(vec![]);
    let mut features: Box<Vec<Feature>> = Box::new(vec![]);

    // create rooms and
    // read in room numbers

    for numbers in 0..num_rooms {
        rooms.push(Room::new(data.remove(0)));
    }

    for room in &*rooms {
        println!("{}", room.get_capacity());
    }

    // create students

    for student in 0..num_students {
        students.push(Student::new());
    }

    // create events

    for event in 0..num_events {
        events.push(Event::new());
    }

    // create features

    for feature in 0..num_features {
        features.push(Feature::new());
    }

    // populate the students with there events
    for aStudent in 0..num_students {
        students[aStudent].addClass()
    }

}
