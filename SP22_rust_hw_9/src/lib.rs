use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
}

impl Student {
    // TODO: Implement this function
    /// Initialize a new student
    pub fn new(name: String) -> Student {
        Student {
            name
        }
    }

    // TODO: Implement this function
    // Create a function that simulates the amount of time a student studies.
    // Print out two messages: "{student} is studying." and "{student} is done studying."
    // Between the two messages put the current thread to sleep for 1000 ms
    pub fn study(&self) {
        println!("{} is studying.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done studying.", self.name);
    }
}

// TODO: Implement this function
// Iterate through the vector of students and pass them onto individual threads
// In each thread, call study();
// Return the handles
pub fn study_progress(students: Vec<Student>) -> Vec<JoinHandle<()>> {
    let mut handle_vec: Vec<JoinHandle<()>> = Vec::new();
    for s in students.iter() {
        let s2 = s.clone();
        let handle = thread::spawn(move || {
            s2.study();
        });
        handle_vec.push(handle);
    }
    return handle_vec;
}
