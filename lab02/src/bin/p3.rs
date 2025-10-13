fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, t: &str) {
    s.push_str(t);
}

fn push_digits_with_underscores(s: &mut String, mut v: i64) {
    if v == 0 {
        s.push('0');
        return;
    }
    let mut digits: Vec<char> = Vec::new();
    while v > 0 {
        let d = (v % 10) as u8;
        digits.push((b'0' + d) as char);
        v /= 10;
    }
    let mut count = 0usize;
    let mut out: Vec<char> = Vec::new();
    for ch in digits {
        if count == 3 {
            out.push('_');
            count = 0;
        }
        out.push(ch);
        count += 1;
    }
    out.reverse();
    for ch in out {
        s.push(ch);
    }
}

fn add_integer(s: &mut String, x: i64) {
    if x < 0 {
        s.push('-');
        push_digits_with_underscores(s, -x);
    } else {
        push_digits_with_underscores(s, x);
    }
}

fn push_unsigned_digits(s: &mut String, mut v: u64) {
    if v == 0 {
        s.push('0');
        return;
    }
    let mut digits: Vec<char> = Vec::new();
    while v > 0 {
        let d = (v % 10) as u8;
        digits.push((b'0' + d) as char);
        v /= 10;
    }
    digits.reverse();
    for ch in digits {
        s.push(ch);
    }
}

fn add_float(s: &mut String, x: f64, decimals: usize) {
    let neg = x.is_sign_negative();
    let a = if neg { -x } else { x };
    let int_part = a as i64;
    let frac = a - int_part as f64;
    let mut pow10: u64 = 1;
    for _ in 0..decimals {
        pow10 *= 10;
    }
    let mut frac_i = (frac * pow10 as f64 + 0.5).floor() as u64;
    let mut int_final = int_part;
    if frac_i >= pow10 {
        frac_i -= pow10;
        int_final += 1;
    }
    if neg {
        s.push('-');
    }
    push_unsigned_digits(s, int_final as u64);
    s.push('.');
    if frac_i == 0 {
        for _ in 0..decimals {
            s.push('0');
        }
    } else {
        let mut tmp = frac_i;
        let mut digits = 1usize;
        while tmp >= 10 {
            digits += 1;
            tmp /= 10;
        }
        if digits < decimals {
            for _ in 0..(decimals - digits) {
                s.push('0');
            }
        }
        push_unsigned_digits(s, frac_i);
    }
}

fn main() {
    let mut s = String::new();

    let template = "\
                                        I 💚
                                        RUST.

    Most            crate      *I*           and     lastest         is
         downloaded        has             downloads     the         version    *F*.
                    ";

    let mut i = 0usize;
    let bytes = template.as_bytes();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b' ' {
            let mut cnt = 0usize;
            while i < bytes.len() && bytes[i] == b' ' {
                cnt += 1;
                i += 1;
            }
            add_space(&mut s, cnt);
        } else if b == b'*' {
            if i + 3 <= bytes.len() && &template[i..i + 3] == "*I*" {
                add_integer(&mut s, 306_437_968);
                i += 3;
            } else if i + 3 <= bytes.len() && &template[i..i + 3] == "*F*" {
                add_float(&mut s, 2.038, 3);
                i += 3;
            } else {
                add_str(&mut s, "*");
                i += 1;
            }
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'*' {
                i += 1;
            }
            let slice = &template[start..i];
            add_str(&mut s, slice);
        }
    }

    print!("{}", s);
}
