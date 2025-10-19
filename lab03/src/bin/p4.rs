#[derive(Debug, PartialEq)]
enum CharError {
    NotAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !(c.is_ascii_graphic() || c == ' ') {
        return Err(CharError::NotPrintable);
    }
    print!("{}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }
    Ok((c as u8 - b'0') as u32)
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if c.is_ascii_digit() {
        return Ok((c as u8 - b'0') as u32);
    }
    let cu = c.to_ascii_uppercase();
    if ('A'..='F').contains(&cu) {
        return Ok((10 + (cu as u8 - b'A')) as u32);
    }
    Err(CharError::NotHexDigit)
}

fn print_error(e: CharError) {
    match e {
        CharError::NotAscii => println!("not ascii"),
        CharError::NotDigit => println!("not digit"),
        CharError::NotHexDigit => println!("not hex digit"),
        CharError::NotLetter => println!("not letter"),
        CharError::NotPrintable => println!("not printable"),
    }
}

fn main() {
    match to_uppercase('a') {
        Ok(x) => println!("{}", x),
        Err(e) => print_error(e),
    }
    match to_uppercase('1') {
        Ok(x) => println!("{}", x),
        Err(e) => print_error(e),
    }
    match to_lowercase('Z') {
        Ok(x) => println!("{}", x),
        Err(e) => print_error(e),
    }
    match print_char('!') {
        Ok(()) => println!(""),
        Err(e) => print_error(e),
    }
    match print_char('\u{0007}') {
        Ok(()) => println!(""),
        Err(e) => print_error(e),
    }
    match char_to_number('8') {
        Ok(v) => println!("{}", v),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(v) => println!("{}", v),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('F') {
        Ok(v) => println!("{}", v),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('g') {
        Ok(v) => println!("{}", v),
        Err(e) => print_error(e),
    }
}
