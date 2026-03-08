use std::io::Write;
use std::{env, io};
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

    verify_weights(&courses);
    print_help();

    let mut command = String::new();

    loop {
        match command.as_str() {
            "q" => break,
            "h" => print_help(),
            "p" => print_courses(&courses),
            "c" => current_results(&courses),
            "n" => needed_results(&courses),
            "s" | "a" => println!("Function not implemented."),
            _ => ()
        }

        print!("> ");
        io::stdout().flush().expect("Could not flush command line.");
        command.clear();
        io::stdin().read_line(&mut command).expect("Could not read command");
        command = command.trim().to_string();
    }
}

fn verify_weights(courses: &Vec<CourseGrades>) {
    for course in courses {
        let mut cumulative_weight = 0.0;
        for assignment in &course.deliverables {
            for _grade in &assignment.grades {
                cumulative_weight += assignment.weight;
            }
        }

        if cumulative_weight < 0.999 || cumulative_weight > 1.001 {
            println!("Warning: For class {}, the assignment weights add to {}", course.code, cumulative_weight);
        }
    }
}

fn print_help() {
    println!("");
    println!("Instructions:");
    println!("");
    println!("Ensure you have a grades.json file in the local folder, or provide a path to a .json as a command line argument.");
    println!("The format of this file is specified in the grades.json_format.md document provided.");
    println!("");
    println!("Available commands:");
    println!("<h> help - prints these instructions");
    println!("<q> quit - exit the program");
    println!("<p> print - prints all ingested courses, use for verifying json parsing");
    println!("<c> see current results - prints the current results in a specified course");
    println!("<n> see the needed grade for each grade boundary in a given course");
    println!("<s> see semester results - not implemented anytime soon");
    println!("<a> see all results - not implemented anytime soon");
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

fn current_results(courses: &Vec<CourseGrades>) {
    let course = find_course(courses);
    let mut secured_grade = 0.0;
    let mut weight_so_far = 0.0;

    for assignment in &course.deliverables {
        for grade in &assignment.grades {
            match grade {
                Some(g) => {
                    secured_grade += g * assignment.weight;
                    weight_so_far += assignment.weight;
                }
                None => ()
            }
        }
    }

    let weighted_grade = secured_grade / weight_so_far;

    println!("So far, completed {}% of course deliverables, secured {}% of the final mark, keeping current average, final grade will be {}%.", weight_so_far*100.0, secured_grade*100.0, weighted_grade*100.0);
}

fn needed_results(courses: &Vec<CourseGrades>) {
    let course = find_course(courses);
    let mut secured_grade = 0.0;
    let mut weight_so_far = 0.0;

    for assignment in &course.deliverables {
        for grade in &assignment.grades {
            match grade {
                Some(g) => {
                    secured_grade += g * assignment.weight;
                    weight_so_far += assignment.weight;
                }
                None => ()
            }
        }
    }

    if weight_so_far >= 1.0 {
        println!("Already secured a grade.");
        return;
    }

    println!("For the following grades, you would need to average the following mark for each grade boundary:\n");

    for boundary in GRADE_BOUNDARIES {
        if boundary.score == 0.0 {continue;}
        print!("{}: ", boundary.letter_grade);
        let mark_needed = (boundary.score - (weight_so_far * secured_grade))/(1.0 - weight_so_far);
        println!("{:.1}%", mark_needed*100.0);
    }
}

fn find_course(courses: &Vec<CourseGrades>) -> &CourseGrades {
    loop {
        print!("Enter course code: ");
        io::stdout().flush().unwrap();

        let mut course_code = String::new();
        io::stdin().read_line(&mut course_code).unwrap();
        course_code = course_code.to_uppercase().trim().to_string();

        let matching_courses: Vec<&CourseGrades> = courses.iter().filter(|c| c.code.contains(&course_code)).collect();

        if matching_courses.is_empty() {println!("No matching course codes found.");}
        else {
            let matching_codes: Vec<&str> = matching_courses.iter().map(|c| c.code.as_str()).collect();
            println!("Matching courses: {:?}", matching_codes);
        }

        if matching_courses.len() == 1 {return matching_courses[0];}
    }
}

struct GradeBoundary {
    score: f64,
    gpa: f64,
    letter_grade: &'static str,
}

const GRADE_BOUNDARIES: [GradeBoundary; 13] = [
    GradeBoundary { score: 0.00, gpa: 0.0, letter_grade: "F"  },
    GradeBoundary { score: 0.50, gpa: 0.7, letter_grade: "D-" },
    GradeBoundary { score: 0.53, gpa: 1.0, letter_grade: "D"  },
    GradeBoundary { score: 0.57, gpa: 1.3, letter_grade: "D+" },
    GradeBoundary { score: 0.60, gpa: 1.7, letter_grade: "C-" },
    GradeBoundary { score: 0.63, gpa: 2.0, letter_grade: "C"  },
    GradeBoundary { score: 0.67, gpa: 2.3, letter_grade: "C+" },
    GradeBoundary { score: 0.70, gpa: 2.7, letter_grade: "B-" },
    GradeBoundary { score: 0.73, gpa: 3.0, letter_grade: "B"  },
    GradeBoundary { score: 0.77, gpa: 3.3, letter_grade: "B+" },
    GradeBoundary { score: 0.80, gpa: 3.7, letter_grade: "A-" },
    GradeBoundary { score: 0.85, gpa: 4.0, letter_grade: "A"  },
    GradeBoundary { score: 0.90, gpa: 4.0, letter_grade: "A+" },
];