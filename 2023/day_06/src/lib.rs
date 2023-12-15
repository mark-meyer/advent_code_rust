fn get_distance(holding: u64, total_time:u64) -> u64 {
    (total_time - holding) * holding
}

fn get_counts(total_time: u64, distance: u64) -> u64 {
    (0..total_time)
    .filter(|&holding_time| get_distance(holding_time, total_time) > distance)
    .count() as u64
} 

pub fn winning_count(times: &Vec<u64>, distances: &Vec<u64>,) -> u64 {
    times
    .iter()
    .zip(distances)
    .map(|(&total_time, &distance)| get_counts(total_time, distance))
    .product()
}