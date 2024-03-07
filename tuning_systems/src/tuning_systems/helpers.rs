pub(crate) fn find_coprimes(num: u32) -> Vec<u32> {
    let mut coprimes = Vec::new();

    for i in 2..num {
        if gcd(num, i) == 1 {
            coprimes.push(i);
        }
    }

    coprimes
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
