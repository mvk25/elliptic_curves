use std::{collections::HashMap, fmt::Error};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Zero};

fn is_quadratic_residue(n: BigInt, p: BigInt) -> bool {
    if &n % &p == BigInt::zero() {
        return true
    }
    BigInt::modpow(&n, &((p.clone() - BigInt::one()) / BigInt::from(2)), &p) == BigInt::one()
}
fn tonelli_shanks(p: BigInt, n: BigInt) -> Option<BigInt> {
    if &n % &p == BigInt::zero() {
        return Some(BigInt::zero())
    }

    if !is_quadratic_residue(n.clone(), p.clone()) {
        return None;
    }
    
    if BigInt::from(4) == BigInt::from(3) {
        return Some(BigInt::modpow(&n, &((&p + BigInt::one()) / BigInt::from(4)), &p));
    }

    let mut q = &p - BigInt::one();
    let mut s = BigInt::zero();

    while &q % 2 == BigInt::zero() {
        s += 1;
        q /= 2;
    }

    let mut z = BigInt::from(2);
    while is_quadratic_residue(z.clone(), p.clone()) {
        z += 1;
    }

    let mut m = s;
    let mut c = BigInt::modpow(&z, &q, &p);
    let mut t = BigInt::modpow(&n, &q, &p);
    let mut r = BigInt::modpow(&n, &((&q + BigInt::one()) / BigInt::from(2)), &p);

    while t.clone() != BigInt::one() {
        let mut i = BigInt::zero();
        let mut temp = t.clone();

        while temp != BigInt::one() {
            i += 1;
            temp = (&temp * &temp) % &p
        }

        let exp: BigInt = m.clone() - i.clone() - BigInt::one();
        // println!("EXP {} {} {}", exp, m.clone(), i);
        let limit = BigInt::from_i128(2_i32.pow(10) as i128).unwrap();
        let pow = BigInt::from(2).modpow(&exp, &limit);
        let b = BigInt::modpow(&c, &pow, &p);
        m = i.clone();
        c = (&b * &b) % p.clone();
        t = (t * &b * &b) % p.clone();
        r = (r * &b) % p.clone();
    }

    Some(r)
}

fn elliptic_equation(x: BigInt, p: BigInt) -> BigInt {
    let a: BigInt = BigInt::from(497);
    let b: BigInt = BigInt::from(1768);

    ((x.clone() * x.clone() * x.clone()) + (a * x) + b) % BigInt::from(p)

}

fn generate_points(slope: BigInt, x_1: BigInt, x_2: BigInt, y_1: BigInt, p: BigInt) -> (BigInt, BigInt) {
    let mut x_3 = (((slope.clone() * slope.clone()) % &p) - (&x_1 + &x_2)) % &p;
    if x_3 < BigInt::zero() {
        x_3 += &p
    }
    let mut y_3 = ((slope.clone() * &x_1) % &p - (slope.clone() * &x_3) % &p - y_1) % &p;
    if y_3 < BigInt::zero() {
        y_3 += &p;
    }
    (x_3, y_3)
}

fn repeater_slope(x_1: BigInt, y_1: BigInt, p: BigInt) -> BigInt {
    let denominator = (BigInt::from(2) * y_1).modinv(&p).unwrap();
    let numerator = ((BigInt::from(3) * (&x_1 * &x_1) % &p) + BigInt::from(497)) % &p;
    return (denominator * numerator) % &p;
}

fn cont_slope(x_1: BigInt, y_1: BigInt, x_2: BigInt, y_2: BigInt, p: BigInt) -> BigInt {
    let numerator = (y_2 - y_1) % &p;
    let denomimator = (x_2 - x_1).modinv(&p).ok_or(Error).map_err(|e| {
        eprintln!("Cyclic group obtained at the last iteration, where x == x of our generator group: {}", e);
    }).unwrap();

    (numerator * denomimator) % &p
    
}
fn main() {
    // E : Y2 = X3 + 497X + 1768 mod 9739
    let p: BigInt = "9739".parse().unwrap();
    let mut table: HashMap<usize, Option<(BigInt, BigInt)>>= HashMap::new();
    let zero = BigInt::zero();
    let one = BigInt::one();
    let mut i = zero.clone();

    let mut elements = vec![];
    while i < p {
        elements.push(elliptic_equation(i.clone(), p.clone()));
        i += &one;
    }

    for (idx, i) in elements.iter().enumerate() {
        if let Some(m) = tonelli_shanks(p.clone(), i.clone()) {
            table.insert(idx, Some((m.clone(), p.clone() - m.clone())));
        }
    }

    let (g_x, g_y) = (BigInt::from(8045), BigInt::from(6936));
    let slope = repeater_slope(g_x.clone(), g_y.clone(), p.clone());
    let mut x_new = BigInt::from(2);
    let mut y_new = BigInt::from(2);

    for i in 0..9739 {
        if i == 0 {
            (x_new, y_new) = generate_points(slope.clone(), g_x.clone(), g_x.clone(), g_y.clone(), p.clone());
            println!("{} {}", x_new, y_new);
            continue;
        }
        let cont_slope = cont_slope(g_x.clone(), g_y.clone(), x_new.clone(), y_new.clone(), p.clone());
        (x_new, y_new) = generate_points(cont_slope.clone(), g_x.clone(), x_new.clone(), g_y.clone(), p.clone());
        println!("{} {}", x_new, y_new);
    }
}