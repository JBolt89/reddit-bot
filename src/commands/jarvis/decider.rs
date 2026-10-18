pub fn decide(content: &[u8], within: u32) -> u32 {
    let mut t = 0;
    let mut n = 0;
    let mut u = false;
    let mut x = 0xcd90f30d;
    for c in content {
        let c = *c as u32;
        n = n << 8 | c;
        t += 1;

        if t == 4 {
            t = 0;
            if u {
                x = x << 1 ^ n;
            } else {
                x = x >> 1 ^ n;
            }
            u = !u;
        }
    }
    x ^= n;
    x = x.wrapping_mul(0x3b1a90f5).wrapping_add(0x144e63d4);
    x % within
}
