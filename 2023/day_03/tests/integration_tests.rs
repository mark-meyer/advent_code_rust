use day_03;

const PUZZLE:&'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_part_one() {
    let lines:Vec<&str> = PUZZLE.lines().collect();
    let test_result = day_03::part_one(&lines);
    assert_eq!(test_result, 4361)
}

#[test]
fn test_part_two() {
    let lines:Vec<&str> = PUZZLE.lines().collect();
    let test_result = day_03::part_two(&lines);
    assert_eq!(test_result, 467835)
}
