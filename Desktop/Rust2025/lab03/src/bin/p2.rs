fn add_checked_panic(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(v) => v,
        None => panic!("overflow add {} + {}", a, b),
    }
}

fn mul_checked_panic(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(v) => v,
        None => panic!("overflow mul {} * {}", a, b),
    }
}

fn main() {
    let x = add_checked_panic(10, 20);
    println!("{}", x);
    let y = mul_checked_panic(7, 6);
    println!("{}", y);
    let _z = add_checked_panic(u32::MAX, 1);
}
