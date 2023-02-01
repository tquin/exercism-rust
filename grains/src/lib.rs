#[allow(unused_parens)]
pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    2u64.pow(s - 1)
}

#[allow(unused_parens)]
pub fn total() -> u64 {
    let mut result: u64 = 0;
    for i in (1..65) {
        result += square(i);
    }
    result
}
