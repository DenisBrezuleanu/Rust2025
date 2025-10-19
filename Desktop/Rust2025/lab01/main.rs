//ambele probleme in acelasi cod
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
//euclid
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
fn are_coprime(a: u32, b: u32) -> bool {
    if a == 0 && b == 0 {
        return false; 
    }
    gcd(a, b) == 1
}

fn main() {
    println!("nr prime intre 1si 100:");
    for n in 0..=100 {
        if is_prime(n) {
            println!("{}", n);
        }
    }
    println!("\n");
    println!("perechile coprime: ");
    for a in 0..=100 {
        for b in 0..=100 {
            if are_coprime(a, b) {
                println!("({}, {})", a, b);
            }
        }
    }
}
