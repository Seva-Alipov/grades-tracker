use std::env;
use std::fs::File;
use std::str::FromStr;
use std::fmt;
use serde::Deserialize;
use serde::de::{self, Deserializer};

#[derive(Debug, Deserialize)]
struct CourseGrades {
    #[serde(rename = "Course code")]
    code: String,

    #[serde(rename = "Course name")]
    name: String,

    #[serde(rename = "Semester")]
    semester: Semester,

    #[serde(rename = "Deliverables")]
    deliverables: Vec<Deliverable>
}

fn default_grades() -> Vec<Option<f64>> {
    vec![None]
}

#[derive(Debug, Deserialize, Clone)]
struct Deliverable {
    name: String,
    weight: f64,
    #[serde(default = "default_grades")]
    grades: Vec<Option<f64>>
}

#[derive(Debug)]
enum Term {
    Fall,
    Summer,
    Winter
}

impl fmt::Display for Term{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Term::Fall => "Fall",
            Term::Winter => "Winter",
            Term::Summer => "Summer"
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]
struct Semester {
    term: Term,
    year: u16
}

impl FromStr for Semester {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {return Err("Semester string too short.".into());}

        let (term_char, year_str) = s.split_at(1);

        let term = match term_char {
            "F" | "f" => Term::Fall,
            "W" | "w" => Term::Winter,
            "S" | "s" => Term::Summer,
            _ => return Err("Invalid semester value".into())
        };

        let mut year: u16 = year_str.parse().map_err(|_| "Error getting semester year".to_string())?;
        year += 2000;

        Ok(Semester { term, year })
    }
}

impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.term, self.year)
    }
}

impl<'de> Deserialize<'de> for Semester {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

fn main() {
    let default_path = String::from("./grades.json");
    let path = env::args().nth(1).unwrap_or(default_path);
    println!("Using file: {}", path);

    let file = File::open(&path).expect("Could not open grades file.");
    let courses: Vec<CourseGrades> = serde_json::from_reader(file).expect("Error parsing json");

    print_courses(&courses);

}

fn print_courses(courses: &Vec<CourseGrades>) {
    for course in courses {
        println!("Read following course:");
        println!("{} - {}, taken {}", course.code, course.name, course.semester);
        println!("  Assignments:");
        for assignment in &course.deliverables {
            println!("    {} worth {} each - {:?}", assignment.name, assignment.weight, assignment.grades);
        }
    }
}