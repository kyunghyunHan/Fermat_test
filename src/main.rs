use rand::distributions::{Distribution, Uniform};
use rand::Rng;
/*
x:base
e:exponents
m:mod


*/
fn n_power(mut x: i32, mut e: i32, m: i32) -> i32 {
    let mut res: i32 = 1;
    //e가 0보다 클떄까지
    while e > 0 {
        //e가 2로 나누어 떨어지면
        if (e % 2) == 1 {
            res = (res * x) % m;
            e = e - 1;
        //아니면
        } else {
            //2=(2*2)%2
            x = (x * x) % m;
            e = e / 2;
        }
    }
    res
}

fn is_bool(n: i32, k: i32) -> bool {
    //n이 2로 나누어 떨어지거나 n이 2가 아니면 false
    if n % 2 == 0 && n != 2 {
        return false;
        //n이 3보다 같거나 3보다 작으면 true
    } else if n <= 3 {
        return true;
    }
    //k가 0보다 크면
    while k > 0 {
        let mut rng = rand::thread_rng();
        let test: i32 = rng.gen_range(1..10);
        let a: i32 = test * (n - 3) + 2;
        println!("{}", a);
        //계산한 갓이 1이 아니면 false
        if n_power(a, n - 1, n) != 1 {
            return false;
        }
    }
    //true리턴
    return true;
}

fn main() {
    is_bool(1, 0);
    let num = rand::thread_rng().gen_range(0..1);
    println!("{:?}", is_bool(1, 2));
    println!("{:?}", num);
}
