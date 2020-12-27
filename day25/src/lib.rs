const SUBJECT_NUMBER: u64 = 7;
const DIVISOR: u64 = 20201227;

fn find_loop_size(pubkey: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = 1;
    while value != pubkey {
        value = (value * SUBJECT_NUMBER) % DIVISOR;
        loop_size += 1;
    }
    loop_size
}

fn transform(pubkey: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 1..=loop_size {
        value = (value * pubkey) % DIVISOR;
    }
    value
}

pub fn part1(pubkey1: u64, pubkey2: u64) -> u64 {
    transform(pubkey2, find_loop_size(pubkey1))
}
