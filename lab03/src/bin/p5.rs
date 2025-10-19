fn find_index(xs: &[i32], target: i32) -> Option<usize> {
    let mut i = 0usize;
    while i < xs.len() {
        if xs[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[derive(Debug)]
enum ParseError {
    Empty,
    NotNumber,
}

fn parse_signed(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut neg = false;
    if bytes[0] == b'-' {
        neg = true;
        i = 1;
    }
    if i == bytes.len() {
        return Err(ParseError::NotNumber);
    }
    let mut v: i32 = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b < b'0' || b > b'9' {
            return Err(ParseError::NotNumber);
        }
        let d = (b - b'0') as i32;
        v = v * 10 + d;
        i += 1;
    }
    if neg {
        v = -v;
    }
    Ok(v)
}

fn main() {
    let data = [3, 1, 4, 1, 5, 9];
    match find_index(&data, 5) {
        Some(i) => println!("{}", i),
        None => println!("not found"),
    }
    match find_index(&data, 7) {
        Some(i) => println!("{}", i),
        None => println!("not found"),
    }
    match parse_signed("-203") {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    }
    match parse_signed("12x") {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    }
}
