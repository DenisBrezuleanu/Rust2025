#[derive(Debug, PartialEq)]
enum MathError {
    Overflow,
}

fn add_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(v) => Ok(v),
        None => Err(MathError::Overflow),
    }
}

fn mul_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(v) => Ok(v),
        None => Err(MathError::Overflow),
    }
}

fn try_ops(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let s = add_checked(a, b)?;
    let p = mul_checked(s, c)?;
    Ok(p)
}

fn main() {
    match try_ops(10, 20, 3) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    }
    match try_ops(u32::MAX, 1, 2) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    }
    match mul_checked(u32::MAX, 2) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{:?}", e),
    }
}
