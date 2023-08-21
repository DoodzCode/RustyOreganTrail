
#[derive(Debug)]
pub struct Person {
    name: String,
    personality_type: String,
    proffession: String,
    status_flags: Vec<String>,
}

impl Person {
    pub fn create_test_object(name: String) -> Person {
        Person {
            name: String::from(name),
            personality_type: String::from("None"),
            proffession: String::from("carpenter"),
            status_flags: Vec::new(),
        }    
    }
}

// TODO rename this struct: we now have rates other than gathering
#[derive(Debug)]
pub struct GatherRates {
    pub food: u8,
    pub water: u8,
    pub wood: u8,
    pub repair: u8,
    pub morale: i8,
}


#[derive(Debug)]
pub struct People {
    // pub population: u8,
    pub morale: i8,
    pub people: Vec<Person>,
}

impl People {
    pub fn create_test_object() -> People {
        let mut test_object: People = People {
            morale: 50,
            people: Vec::new(),
        };

        let mut counter: i32 = 1;
        loop {
            let name = format!("person_{}", counter);
            let new_person = Person::create_test_object(name);
            test_object.add_person(new_person);
            counter += 1;

            if counter > 20 {
                break;
            }
        }

        test_object
    }

    pub fn add_person(&mut self, new_person: Person) {
        self.people.push(new_person);
    }

    // TODO
    // pub fn remove_person(){}

    pub fn population(&self) -> u8 {
        let population: u8 = self.people.len().try_into().unwrap();
        population
    }
}
