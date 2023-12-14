use day_05::{part_one, part_two, parse_input};

#[test]
fn test_part_one(){
    let (seeds, convs) = parse_input::parse_file("test_input.txt").expect("could not parse file");
    let answer1 = part_one(&seeds, &convs);
    assert_eq!(answer1, Some(35));
}
#[test]
fn test_part_two(){
    let (seeds, convs) = parse_input::parse_file("test_input.txt").expect("could not parse file");
    let answer1 = part_two(&seeds, &convs);
    assert_eq!(answer1, Some(46));
}
