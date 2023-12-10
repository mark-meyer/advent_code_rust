const  NUMBERS: [(&'static str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];


pub fn solve_part_one(s: &str) -> u32 {
    s.split("\n")
    .map(|line| line.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
    )
    .map(|s| first_last_digits(&s))
    .sum()
}

pub fn solve_part_two(s: &str) -> u32 {
    s.split("\n")
    .map(|line| first_last_words(&line))
    .sum()
}

fn first_last_digits(s: &str) -> u32 {
    let chars:Vec<u32> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    10 * chars.first().unwrap() + chars.last().unwrap()
} 

fn first_last_words(s: &str) -> u32 {
    // Sorts None() before Some() so we unwrap the option for min since None() will be the min
    // but don't need to for max, since Some(max_found) will be the max
    let left = NUMBERS
        .iter()
        .min_by_key(|t| s.find( t.0 )
            .unwrap_or(usize::MAX)
        );

    let right = NUMBERS
        .iter()
        .max_by_key(|t| s.rfind( t.0 ));

    10 * left.unwrap().1 + right.unwrap().1

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(28, first_last_digits("fhj2fjf8"));
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(88, first_last_digits("8"));
    }

    #[test]
    fn test_words() {
        assert_eq!(28, first_last_words("fhjtwofjfeightl"));
    }

    #[test]
    fn test_words_numbers() {
        assert_eq!(29, first_last_words("fhjtwofjfeightl9"));
        assert_eq!(38, first_last_words("fh3jtwofjfeightl"));
    }

    #[test]
    fn test_elided_word() {
        assert_eq!(21, first_last_words("vnmtwoneqq"));
    }
    #[test]
    fn test_single_word() {
        assert_eq!(88, first_last_words("eight"));
    }

}