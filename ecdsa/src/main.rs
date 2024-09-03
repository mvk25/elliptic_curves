// We are going to use the bitcoin secp256k1 curve
// The elliptic curve domain parameters over Fp associated with a Koblitz curve secp256k1 are specified by the sextuple T = (p,a,b,G,n,h) where the finite field Fp is defined by:

//     p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
//     = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1

// The curve E: y2 = x3+ax+b over Fp is defined by:

//     a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
//     b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007

// The base point G in compressed form is:

//     G = 02 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798

// and in uncompressed form is:

//     G = 04 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
// Finally the order n of G and the cofactor are:

//     n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
//     h = 01


use num_bigint::{BigInt, RandomBits, Sign};
use num_traits::{zero, One, Zero};
use rand::Rng;
use crypto::{digest::Digest, sha1::Sha1};

const G: &str = "04 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8";
const P: &str = "FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F";
const N: &str = "FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141";


fn hex_to_bigint(hex_str: &str) -> (BigInt, BigInt) {
    let x: Vec<u8> = (0..hex_str.len() / 2).into_iter().step_by(2).map(|i| {
        u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap()
    }).collect();

    let y: Vec<u8> = (hex_str.len() / 2..hex_str.len()).into_iter().step_by(2).map(|i| {
        u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap()
    }).collect();
    (BigInt::from_bytes_be(Sign::Plus, &x), BigInt::from_bytes_be(Sign::Plus, &y))
}

fn hex_to_bigint2(hex_str: &str) -> BigInt {
    let int: Vec<u8> = (0..hex_str.len()).into_iter().step_by(2).map(|i| {
        u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap()
    }).collect();

    BigInt::from_bytes_be(Sign::Plus, &int)
}

fn add_two_points(point_p: (BigInt, BigInt), point_q: (BigInt, BigInt), p: BigInt) -> (BigInt, BigInt) {
    let slope: BigInt;
    if &point_p.0 == &point_q.0 && &point_p.1 == &point_q.1 {
        slope = (BigInt::from(3) * &point_p.0 * &point_q.0) * (BigInt::from(2) * &point_p.1).modinv(&p).unwrap();
    } else {
        slope = (&point_q.1 - &point_p.1) * (&point_q.0 - &point_p.0).modinv(&p).unwrap();
    }

    let mut x_new: BigInt = (&slope * &slope - &point_p.0 - &point_q.0) % &p;
    if x_new < zero() {
        x_new += &p;
    }
    let mut y_new: BigInt = (&slope * (&point_p.0 - &x_new) - &point_p.1) % &p;
    if y_new < zero() {
        y_new += &p;
    }


    (x_new, y_new)
}

fn bigint_to_binary(num: BigInt) -> String {
    let zero = BigInt::zero();
    let two:  BigInt = "2".parse().unwrap();
    let one = BigInt::one();
    let mut e = num;
    let mut binary_string: String = String::from("");
    while &e > &zero {
        if &e % &two == one {
            binary_string.push_str("1")
        } else {
            binary_string.push_str("0");
        }
        e /= two.clone();
        
    }

    binary_string
}

fn double_and_add(generator: (BigInt, BigInt), k: BigInt, p: BigInt) -> (BigInt, BigInt) {
    let mut target_point = generator.clone();

    let mut k_binary = bigint_to_binary(k);
    k_binary = k_binary.chars().rev().collect();
    for i in 1..k_binary.len() {
        let current = &k_binary[i..i + 1];
        target_point = add_two_points(target_point.clone(), target_point.clone(), p.clone());

        if current == "1" {
            target_point = add_two_points(target_point.clone(), generator.clone(), p.clone());
        }

    };
    target_point

}

fn main() {
    let a = BigInt::zero();
    let b: BigInt = "7".parse().unwrap();
    
    let _g: String = G[2..].replace(" ", "").to_lowercase();
    
    let _p: BigInt = hex_to_bigint2(&P.replace(" ", "").to_lowercase());
    
    let _n: BigInt = hex_to_bigint2(&N.replace(" ", "").to_lowercase());

    let mut rng = rand::thread_rng();

    let generator = hex_to_bigint(&_g);

    // Private Key
    let d: BigInt= rng.sample(RandomBits::new(256));
    println!("D {}", d);
    // Public Key
    let q = double_and_add(generator.clone(), d.clone(), _p.clone());


    let random_key: BigInt = rng.sample(RandomBits::new(256));
    let random_point = double_and_add(generator.clone(), random_key.clone(), _p.clone());
    

    let message = b"Here we are at the apex of the dilemma";
    let mut hasher = Sha1::new();
    hasher.input(message);
    let hex = hasher.result_str();

    let hash_int = hex_to_bigint2(&hex);
    println!("{}", hash_int);
    let r = random_point.0 % &_n;
    println!("r: {r}\n");

    let mut s = ((&hash_int + (&r * &d) % &_n) * random_key.modinv(&_n).unwrap()) % &_n;
    if s < zero() {
        s += &_n;
    }

    println!("s: {s}\n");


    // Verificacion
    let w = s.modinv(&_n).unwrap();

    let xc = (&hash_int * &w) % &_n;
    println!("xc: {xc}");
    let u1 = double_and_add(generator.clone(), xc.clone(), _p.clone());
    println!("u1: {:?}\n", u1);
    let xd = (&r * &w) % &_n;
    println!("xd: {xd}");

    let u2 = double_and_add(q, xd.clone(), _p.clone());
    println!("u2: {:?}\n", u2);
    let checker = add_two_points(u1, u2, _p.clone());

    println!("{:?}", checker);
    println!("r: {r}");

    println!("{} {}", checker.0, r);


}

