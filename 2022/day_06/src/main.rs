use std::collections::HashSet;
use std::fs;
use std::path::Path;

static DATA:&str = "data.txt";

fn find_first_marker(s:&str, marker_size:usize) -> usize {
    let tokens:Vec<char> = s.chars().collect();
    let mut h = HashSet::<char>::with_capacity(marker_size);

    for (i, w) in tokens.windows(marker_size).enumerate() {
        h.extend(w);
        if h.len() == marker_size {
            return i + marker_size
        }
        h.clear()
    }

    panic!("No marker found!")
}

fn main() {
    let path = Path::new(DATA);
    let s = fs::read_to_string(path).expect("could not open the data file");

    let part_one = find_first_marker(&s, 4);   
    println!("Part one: {}", part_one);

    let part_two = find_first_marker(&s, 14);   
    println!("Part two: {}", part_two);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let loc = find_first_marker(s, 4);
        assert_eq!(loc, 5);

        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let loc = find_first_marker(s, 4);
        assert_eq!(loc, 6);
        
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let loc = find_first_marker(s, 4);
        assert_eq!(loc, 10);

        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let loc = find_first_marker(s, 4);
        assert_eq!(loc, 11);
    }

    #[test]
    fn test_start_of_message() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let loc = find_first_marker(s, 14);
        assert_eq!(loc, 23);

        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let loc = find_first_marker(s, 14);
        assert_eq!(loc, 23);
        
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let loc = find_first_marker(s, 14);
        assert_eq!(loc, 29);

        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let loc = find_first_marker(s, 14);
        assert_eq!(loc, 26);
    }
}