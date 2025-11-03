use std::fs;

struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn parse_line(line: &str) -> Option<Student> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let name = parts[0].trim().to_string();
    let phone = parts[1].trim().to_string();
    let age: u32 = parts[2].trim().parse().ok()?;
    Some(Student { name, phone, age })
}

fn main() {
    let content = fs::read_to_string("students.csv").expect("students.csv");
    let mut students: Vec<Student> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(s) = parse_line(line) {
            students.push(s);
        }
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
