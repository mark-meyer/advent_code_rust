use day_01;

const INPUT_1: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";


const INPUT_2: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn example1() {
    assert_eq!(day_01::solve_part_one(INPUT_1), 142);
}

#[test]
fn example2() {
    assert_eq!(day_01::solve_part_two (INPUT_2), 281);
}