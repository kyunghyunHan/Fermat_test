use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn n_power(mut x: i32, mut e: i32, m: i32) -> i32 {
    let mut res: i32 = 1;

    while e > 0 {
        if (e % 2) == 1 {
            res = (res * x) % m;
            e = e - 1;
        } else {
            x = (x * x) % m;
            e = e / 2;
        }
    }
    res
}

fn is_bool(n: i32, k: i32) -> bool {
    if n % 2 == 0 && n != 2 {
        return false;
    } else if n <= 3 {
        return true;
    }
    while k > 0 {
        let mut rng = rand::thread_rng();
        let test: i32 = rng.gen_range(0..1);
        let a: i32 = test * (n - 3) + 2;
        println!("{}", a);

        if n_power(a, n - 1, n) != 1 {
            return false;
        }
    }

    return true;
}

fn main() {
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
}
