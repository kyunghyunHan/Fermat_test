use rand::Rng;
/*
k 값이 클수록 정답일 확률을 나타낸다
- 복합 입력에 대한  결과가 높아진다 ,소수를 위해
- 입력,결과를 항상정확함
- 다음 k번 반복
- a[2,n-2]범위에서 임으로 선택
- b)gcd(a,n)!=1이면 false를 반환
- c)n-1 !==1(mod n) 그런다음 fasle 를 반환
2t true아마도 프라임 반환

*/
fn power(mut a: i32, mut n: i32, p: i32) -> i32 {
    let mut res = 1;

    a = a % p;

    while n > 0 {
        if n == 1 {
            res = (res * a) % p;
        }
        n = n / 2;
        a = (a * a) % p;
    }
    return res;
}
fn gcd(a: i32, b: i32) -> i32 {
    if a < b {
        return gcd(b, a);
    } else if a % b == 0 {
        return b;
    } else {
    }
    return gcd(b, a % b);
}

fn prime(n: i32, mut k: i32) -> bool {
    if n <= 1 || n == 4 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    while k > 0 {
        let num: f64 = rand::thread_rng().gen_range(0.0..1.0);

        let a = 2 as f64 + num % (n as f64 - 4 as f64);
        // println!("{}", a);
        if gcd(n, a as i32) != 1 {
            // println!("{}", a);
            return false;
        }
        if power(a as i32, n - 1, n) != 1 {
            return false;
        }
        k = k - 1
    }
    return true;
}

fn main() {
    let k = 11540;
    println!("{}", prime(1, k));
    println!("{}", prime(2, k));
    println!("{}", prime(3, k));
    println!("{}", prime(4, k));
    println!("{}", prime(5, k));
    println!("{}", prime(6, k));
    println!("{}", prime(7, k));
    println!("{}", prime(8, k));
    println!("{}", prime(9, k));
    println!("{}", prime(10, k));
    println!("11{}", prime(11, k));
    println!("12{}", prime(12, k));
    println!("13{}", prime(13, k));
    println!("{}", prime(14, k));
    println!("{}", prime(15, k));
    println!("{}", prime(16, k));
    println!("{}", prime(17, k));
}
