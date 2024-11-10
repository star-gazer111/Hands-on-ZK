fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn tonelli_shanks(n: u64, p: u64) -> Option<u64> {
    if mod_pow(n, (p - 1) / 2, p) != 1 {
        return None; // No solution exists if n is not a quadratic residue modulo p
    }

    if p == 2 {
        return Some(n % 2);
    }

    if p % 4 == 3 {
        return Some(mod_pow(n, (p + 1) / 4, p));
    }

    // Find q and s such that p - 1 = q * 2^s with q odd
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Find a non-residue z modulo p
    let mut z = 2;
    while mod_pow(z, (p - 1) / 2, p) == 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(n, q, p);
    let mut r = mod_pow(n, (q + 1) / 2, p);

    while t != 0 && t != 1 {
        let mut i = 1;
        let mut temp = mod_pow(t, 2, p);
        while temp != 1 {
            temp = mod_pow(temp, 2, p);
            i += 1;
        }

        let b = mod_pow(c, 1 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }

    Some(r)
}

fn main() {
    let n = 10;
    let p = 13;

    match tonelli_shanks(n, p) {
        Some(root) => println!("Square root of {} modulo {} is {}", n, p, root),
        None => println!("No square root exists for {} modulo {}", n, p),
    }
}

