#[allow(dead_code)]
fn find_coprimes(num: i32) -> Vec<i32> {
    let mut coprimes = Vec::new();

    for i in 2..num {
        if gcd(num, i) == 1 {
            coprimes.push(i);
        }
    }

    coprimes
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
