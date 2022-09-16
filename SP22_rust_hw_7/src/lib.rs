/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub netid: String,
    pub schedule: Schedule,
}

/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub credit_hours: u8,
}

/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    pub classes: Vec<Class>,
    pub credit_hours: u8,
}

impl Student {
    // TODO: Implement this function
    /// Initialize a new student
    /// Set the students schedule with a new empty schedule
    pub fn new(name: String, netid: String) -> Student {
        Student {
            name,
            netid,
            schedule: Schedule { classes: Vec::new(), credit_hours: 0}
        }
    }

    // TODO: Implement this function
    /// Enroll a student in a schedule
    pub fn schedule_enrollment(&mut self, schedule: Schedule) {
        self.schedule = schedule;
    }

    // TODO: Implement this function
    /// Check if a student is a classmate of another student
    pub fn is_classmate(&self, other: &Student) -> bool {
        for my_class in self.schedule.classes.iter() {
            for other_class in other.schedule.classes.iter() {
                if my_class == other_class {
                    return true;
                }
            }
        }
        false
    }
}

impl Class {
    // TODO: Implement this function
    /// Initialize a new class
    pub fn new(name: String, credit_hours: u8) -> Class {
        Class {
            name,
            credit_hours
        }
    }
}

impl Schedule {
    // TODO: Implement this function
    /// Initialize a new schedule
    /// Credit hours should be total up from the classes
    pub fn new(classes: Vec<Class>) -> Schedule {
        let mut total_hours: u8 = 0;
        for c in classes.iter() {
            total_hours += c.credit_hours;
        }
        Schedule {
            classes,
            credit_hours: total_hours
        }
    }
}