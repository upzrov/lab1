use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use regex::Regex;

use crate::models::{Entity, Gardener, Gender, Seller, Student};

pub trait Repository {
    fn read_all(&self, path: &Path) -> Result<Vec<Entity>, Box<dyn Error>>;
    fn append(&self, path: &Path, e: &Entity) -> Result<(), Box<dyn Error>>;
    fn overwrite_all(&self, path: &Path, all: &[Entity]) -> Result<(), Box<dyn Error>>;
}

pub struct FileRepository;

impl FileRepository {
    pub fn new() -> Self {
        Self
    }

    fn parse_attrs(block: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut map = HashMap::new();
        let re = Regex::new(r#""([^"]+)"\s*:\s*"([^"]*)""#)?;
        for cap in re.captures_iter(block) {
            map.insert(cap[1].to_string(), cap[2].to_string());
        }
        Ok(map)
    }

    fn write_header(f: &mut File, kind: &str, first: &str, last: &str) -> std::io::Result<()> {
        writeln!(f, "{} {}{}", kind, first, last)
    }

    fn write_field(f: &mut File, key: &str, value: &str, last: bool) -> std::io::Result<()> {
        if last {
            writeln!(f, "\"{}\": \"{}\"}};", key, value)
        } else {
            writeln!(f, "\"{}\": \"{}\",", key, value)
        }
    }
}

impl Repository for FileRepository {
    fn read_all(&self, path: &Path) -> Result<Vec<Entity>, Box<dyn Error>> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut entities = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break; // EOF
            }
            let header = line.trim();
            if header.is_empty() {
                continue;
            }

            let mut parts = header.split_whitespace();
            let kind = parts.next().ok_or("Bad header")?;
            let mut block = String::new();

            loop {
                let mut block_line = String::new();
                if reader.read_line(&mut block_line)? == 0 {
                    break;
                }
                block.push_str(&block_line);
                if block_line.contains("};") {
                    break;
                }
            }

            let attrs = Self::parse_attrs(&block)?;
            match kind {
                "Student" => {
                    let student = Student {
                        first_name: attrs
                            .get("firstName")
                            .ok_or("student firstName missing")?
                            .clone(),
                        last_name: attrs
                            .get("lastName")
                            .ok_or("student lastName missing")?
                            .clone(),
                        student_id: attrs.get("studentId").ok_or("studentId missing")?.clone(),
                        gender: attrs
                            .get("gender")
                            .map(|s| Gender::from_str(s))
                            .unwrap_or(Gender::Other),
                        course: attrs
                            .get("course")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(1),
                        dormitory_room: attrs.get("dorm").cloned(),
                    };
                    entities.push(Entity::Student(student));
                }
                "Seller" => {
                    let seller = Seller {
                        first_name: attrs
                            .get("firstName")
                            .ok_or("seller firstName missing")?
                            .clone(),
                        last_name: attrs
                            .get("lastName")
                            .ok_or("seller lastName missing")?
                            .clone(),
                        gender: attrs
                            .get("gender")
                            .map(|s| Gender::from_str(s))
                            .unwrap_or(Gender::Other),
                        shop: attrs.get("shop").cloned(),
                    };
                    entities.push(Entity::Seller(seller));
                }
                "Gardener" => {
                    let gardener = Gardener {
                        first_name: attrs
                            .get("firstName")
                            .ok_or("gard firstName missing")?
                            .clone(),
                        last_name: attrs
                            .get("lastName")
                            .ok_or("gard lastName missing")?
                            .clone(),
                        gender: attrs
                            .get("gender")
                            .map(|s| Gender::from_str(s))
                            .unwrap_or(Gender::Other),
                        experience_years: attrs.get("experience").and_then(|s| s.parse().ok()),
                    };
                    entities.push(Entity::Gardener(gardener));
                }
                other => eprintln!("Unknown type in file: {}", other),
            }
        }
        Ok(entities)
    }

    fn append(&self, path: &Path, e: &Entity) -> Result<(), Box<dyn Error>> {
        let mut f = OpenOptions::new().create(true).append(true).open(path)?;

        match e {
            Entity::Student(s) => {
                Self::write_header(&mut f, "Student", &s.first_name, &s.last_name)?;
                writeln!(f, "{{ \"firstName\": \"{}\",", s.first_name)?;
                writeln!(f, "\"lastName\": \"{}\",", s.last_name)?;
                writeln!(f, "\"studentId\": \"{}\",", s.student_id)?;
                writeln!(f, "\"gender\": \"{:?}\",", s.gender)?;
                writeln!(f, "\"course\": \"{}\",", s.course)?;
                if let Some(d) = &s.dormitory_room {
                    Self::write_field(&mut f, "dorm", d, true)?;
                } else {
                    writeln!(f, "}};")?;
                }
            }
            Entity::Seller(s) => {
                Self::write_header(&mut f, "Seller", &s.first_name, &s.last_name)?;
                writeln!(f, "{{ \"firstName\": \"{}\",", s.first_name)?;
                writeln!(f, "\"lastName\": \"{}\",", s.last_name)?;
                writeln!(f, "\"gender\": \"{:?}\",", s.gender)?;
                if let Some(shop) = &s.shop {
                    Self::write_field(&mut f, "shop", shop, true)?;
                } else {
                    writeln!(f, "}};")?;
                }
            }
            Entity::Gardener(g) => {
                Self::write_header(&mut f, "Gardener", &g.first_name, &g.last_name)?;
                writeln!(f, "{{ \"firstName\": \"{}\",", g.first_name)?;
                writeln!(f, "\"lastName\": \"{}\",", g.last_name)?;
                writeln!(f, "\"gender\": \"{:?}\",", g.gender)?;
                if let Some(exp) = g.experience_years {
                    Self::write_field(&mut f, "experience", &exp.to_string(), true)?;
                } else {
                    writeln!(f, "}};")?;
                }
            }
        }
        Ok(())
    }

    fn overwrite_all(&self, path: &Path, all: &[Entity]) -> Result<(), Box<dyn Error>> {
        let mut f = File::create(path)?;
        for e in all {
            // just reuse append logic, but with an already-opened file
            drop(f); // close previous handle to avoid borrow conflict
            self.append(path, e)?;
            f = OpenOptions::new().append(true).open(path)?;
        }
        Ok(())
    }
}
