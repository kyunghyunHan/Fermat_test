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
fn modulo(base: i32, mut e: i32, m: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b = base.clone();

    while e > 0 {
        if e % 2 == 1 {
            a = (a * b) % m;
        }
        b = (b * b) % m;
        e = e / 2;
    }
    return a % m;
}

fn fermat(m: i32, it: i32) -> bool {
    if m == 1 {
        return false;
    }
    for i in 0..it {
        let num: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let x = num % (m as f64 - 1 as f64) + 1 as f64;
        println!("{}", x);
        if modulo(x as i32, m - 1, m) != 1 {
            println!("{}", modulo(x as i32, m - 1, m));
            return false;
        }
    }
    return true;
}

fn main() {
    let it = 50;
    let num = 479001599;

    if fermat(num, it) {
        println!("{}", "is prime");
    } else {
        println!("{}", "is not prime")
    }
}
