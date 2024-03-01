pub(crate) fn find_coprimes(num: usize) -> Vec<usize> {
    let mut coprimes = Vec::new();

    for i in 2..num {
        if gcd(num as u32, i as u32) == 1 {
            coprimes.push(i as usize);
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
