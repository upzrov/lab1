#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn from_str(s: &str) -> Gender {
        match s.to_lowercase().as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => Gender::Other,
        }
    }
}

pub trait Person {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
    fn gender(&self) -> &Gender;
}

// pub trait Ability {
//     fn sleep_standing();
// }

#[derive(Debug)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub student_id: String,
    pub course: u8,
    pub dormitory_room: Option<String>,
}

impl Person for Student {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }

    fn gender(&self) -> &Gender {
        &self.gender
    }
}

impl Student {
    pub fn study(&mut self) {
        if self.course < 10 {
            self.course += 1;
        }
    }
}

#[derive(Debug)]
pub struct Seller {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub shop: Option<String>,
}

impl Person for Seller {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
    fn gender(&self) -> &Gender {
        &self.gender
    }
}

#[derive(Debug)]
pub struct Gardener {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub experience_years: Option<u8>,
}

impl Person for Gardener {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
    fn gender(&self) -> &Gender {
        &self.gender
    }
}

#[derive(Debug)]
pub enum Entity {
    Student(Student),
    Seller(Seller),
    Gardener(Gardener),
}

impl Entity {
    pub fn as_person(&self) -> &dyn Person {
        match self {
            Entity::Student(s) => s,
            Entity::Seller(s) => s,
            Entity::Gardener(g) => g,
        }
    }
}
