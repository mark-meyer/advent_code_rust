use day_06::*;

fn main() {
    let times_day1:Vec<u64> = vec![54, 81, 70, 88];
    let distances_day1:Vec<u64> = vec![446, 1292, 1035, 1007];

    let times_day2:Vec<u64> = vec![54817088];
    let distances_day2:Vec<u64> = vec![446129210351007];

    let part_one = winning_count(&times_day1, &distances_day1);
    println!("Day One: {}", part_one);

    let part_two = winning_count(&times_day2, &distances_day2);
    println!("Day One: {}", part_two);
}
