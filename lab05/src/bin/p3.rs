use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let content = fs::read_to_string("students_jsonl.txt").expect("students_jsonl.txt");
    let mut students: Vec<Student> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let s: Student = serde_json::from_str(line).expect("bad json");
        students.push(s);
    }
    if students.is_empty() {
        return;
    }
    let mut youngest_index = 0usize;
    let mut oldest_index = 0usize;
    for (i, s) in students.iter().enumerate() {
        if s.age < students[youngest_index].age {
            youngest_index = i;
        }
        if s.age > students[oldest_index].age {
            oldest_index = i;
        }
    }
    let y = &students[youngest_index];
    let o = &students[oldest_index];
    println!("Youngest: {}, {}, {}", y.name, y.phone, y.age);
    println!("Oldest: {}, {}, {}", o.name, o.phone, o.age);
}
