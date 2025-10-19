fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d: u16 = 3;
    while (d as u32) * (d as u32) <= n as u32 {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut c: u32 = x as u32 + 1;
    while c <= u16::MAX as u32 {
        if is_prime(c as u16) {
            return Some(c as u16);
        }
        c += 1;
    }
    None
}

fn main() {
    let mut x: u16 = 0;
    let mut k = 0u32;
    loop {
        match next_prime(x) {
            Some(p) => {
                print!("{} ", p);
                x = p;
                k += 1;
                if k >= 25 {
                    break;
                }
            }
            None => {
                println!("None");
                break;
            }
        }
    }
    println!();
    let y: u16 = u16::MAX - 5;
    match next_prime(y) {
        Some(p) => println!("{}", p),
        None => println!("None"),
    }
}
