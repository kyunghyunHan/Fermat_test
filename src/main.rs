use rand::Rng;

fn modulo(base: i32, mut exponent: i32, mode: i32) -> i32 {
    let mut x = 1;
    let mut y = base;
    while exponent > 0 {
        if exponent % 2 == 1 {
            println!("{}", exponent);
            x = (x * base) % mode;
        }
        y = (y * y) % mode;
        exponent = exponent / 2;
        println!("{}", exponent);
    }
    return x % mode;
}
fn fermat(p: i32, iterations: i32) -> bool {
    if p == 1 {
        return false;
    }
    for i in 0..iterations {
        let num = rand::thread_rng().gen_range(0..1);
        let a = num % (p - 1) + 1;
        if modulo(a, p - 1, p) != 1 {
            return false;
        }
    }
    return true;
}
fn main() {
    if fermat(91, 100) {
        println!("is prime")
    } else {
        println!("is not prime");
    }
}
