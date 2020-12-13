pub type Bus = u64;
pub type Timestamp = u64;

pub fn part1(arrival_time: Timestamp, busses: &[Option<Bus>]) -> Option<u64> {
    busses
        .iter()
        .filter_map(|&bus| bus)
        .map(|bus| {
            let runs = (arrival_time as f32 / bus as f32).ceil() as u64;
            (runs * bus - arrival_time, bus)
        })
        .min()
        .map(|(wait, bus)| wait * bus)
}

pub fn part2(busses: &[Option<Bus>]) -> Timestamp {
    let mut time = 0;
    let mut jump = 1;
    for (freq, offset) in busses
        .iter()
        .zip(0_u64..)
        .filter_map(|(bus, offset)| bus.map(|freq| (freq, offset)))
    {
        while (time + offset) % freq != 0 {
            time += jump;
        }
        // Assuming all frequencies are prime numbers
        jump *= freq;
    }
    time
}
