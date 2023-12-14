use day_05::parse_input;

fn main() {
    let (seeds, convs) = parse_input::parse_file("data.txt").expect("could not parse file");
    if let Some(answer1) = day_05::part_one(&seeds, &convs) {
        println!("Part One: {}", answer1);
    };
    if let Some(answer2) = day_05::part_two(&seeds, &convs) {
        println!("Part Two: {}", answer2);
    };
}
