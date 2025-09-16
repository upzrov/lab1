use std::{error::Error, path::Path, rc::Rc};

use lab::{
    models::{Entity, Gender, Student},
    repository::Repository,
};

pub struct ConsoleMenu {
    repo: Rc<dyn Repository>,
    file_path: String,
}

impl ConsoleMenu {
    pub fn new(repo: Rc<dyn Repository>, file_path: &str) -> Self {
        Self {
            repo,
            file_path: file_path.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.file_path);
        let entities = self.repo.read_all(path)?;
        println!("Loaded {} entities.", entities.len());

        let mut count: usize = 0;
        for e in &entities {
            if let Entity::Student(s) = e {
                if matches!(s.gender, Gender::Male) && s.course == 3 && s.dormitory_room.is_some() {
                    count += 1;
                    println!(
                        "Matched student: {} {} id={}",
                        s.first_name, s.last_name, s.student_id
                    );
                }
            }
        }
        println!("Number of male 3rd-year students living in dorm: {count}");

        let name = "Pupkin";
        println!("Search by lastName {name}");
        for e in &entities {
            if e.as_person().last_name() == name {
                println!("Found: {:?}", e);
            }
        }

        let s = Student {
            first_name: "Vlad".into(),
            last_name: "Upyrov".into(),
            student_id: "3332".into(),
            gender: Gender::Male,
            course: 3,
            dormitory_room: Some("101-12".into()),
        };
        let ent = Entity::Student(s);
        self.repo.append(path, &ent)?;
        println!("Appended student to file.");

        // let g = Gardener {
        //     first_name: "Maksym".into(),
        //     last_name: "Steblovskyi".into(),
        //     gender: Gender::Male,
        //     experience_years: Some(3),
        // };
        // let gardener_ent = Entity::Gardener(g);
        // self.repo.append(path, &gardener_ent)?;
        // println!("Appended gardener to file.");

        Ok(())
    }
}
