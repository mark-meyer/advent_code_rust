#[derive(Debug)]
/// Trie with an alphabet of five letters
/// represnting each color stripe. The
/// goal was to make this more efficient
/// than using a HashMap.
pub struct Trie {
    children: [Option<Box<Trie>>; 5],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: Default::default(),
            is_word: false,
        }
    }

    // Get the internal index of the stripe
    fn index_stripe(stripe: char) -> usize {
        match stripe {
            'b' => 0,
            'g' => 1,
            'r' => 2,
            'u' => 3,
            'w' => 4,
            _ => panic!("That towel stripe is not allowed in this onsen"),
        }
    }

    /// Add a prefix to the Trie
    pub fn insert(&mut self, towel: &str) {
        let mut current = self;

        for stripe in towel.chars() {
            let i = Trie::index_stripe(stripe);
            current = current.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        current.is_word = true;
    }

    /// Get all the known prefixes that match
    /// the start of the towel pattern
    pub fn get_prefixes<'a>(&self, towel: &'a str) -> Vec<&'a str> {
        let mut current = self;
        let mut prefixes: Vec<&str> = vec![];

        for (len, stripe) in towel.chars().enumerate() {
            let i = Trie::index_stripe(stripe);

            if let Some(next) = &current.children[i] {
                if next.is_word {
                    prefixes.push(&towel[..=len])
                }
                current = next
            } else {
                return prefixes;
            }
        }
        prefixes
    }

    /// Returns true if you can make the pattern
    /// on towel with the words in the Trie
    pub fn is_possible(&self, towel: &str) -> bool {
        let n = towel.len();
        let mut memo = vec![false; n + 1];
        memo[0] = true;
        for start in 0..n {
            if memo[start] {
                for prefix in self.get_prefixes(&towel[start..]) {
                    let end = start + prefix.len();
                    memo[end] = true;
                    if n == end {
                        return true;
                    }
                }
            }
        }
        memo[n]
    }

    /// Returns the number of ways you can make
    /// the towel with the words in the Trie
    pub fn count_possible(&self, towel: &str) -> u64 {
        // this is basically identical to is_possible
        // but without the early exit
        let n = towel.len();
        let mut memo = vec![0; n + 1];
        memo[0] = 1;
        for start in 0..n {
            if memo[start] > 0 {
                for prefix in self.get_prefixes(&towel[start..]) {
                    let end = start + prefix.len();
                    if end <= n {
                        memo[end] += memo[start];
                    }
                }
            }
        }
        memo[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert() {
        let mut t = Trie::new();
        t.insert("bwg");
        t.insert("bwgr");

        assert_eq!(t.get_prefixes("bwgrra"), vec!["bwg", "bwgr"]);
    }

    #[test]
    fn test_possible() {
        let mut t = Trie::new();
        ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
            .iter()
            .for_each(|p| t.insert(p));

        assert_eq!(t.is_possible("brwrr"), true);
        assert_eq!(t.is_possible("bggr"), true);
        assert_eq!(t.is_possible("gbbr"), true);
        assert_eq!(t.is_possible("rrbgbr"), true);
        assert_eq!(t.is_possible("ubwu"), false);
        assert_eq!(t.is_possible("bwurrg"), true);
        assert_eq!(t.is_possible("brgr"), true);
        assert_eq!(t.is_possible("bbrgwb"), false);
    }

    #[test]
    fn count_possible() {
        let mut t = Trie::new();
        ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
            .iter()
            .for_each(|p| t.insert(p));

        assert_eq!(t.count_possible("brwrr"), 2);
        assert_eq!(t.count_possible("bggr"), 1);
        assert_eq!(t.count_possible("gbbr"), 4);
        assert_eq!(t.count_possible("rrbgbr"), 6);
        assert_eq!(t.count_possible("ubwu"), 0);
        assert_eq!(t.count_possible("bwurrg"), 1);
        assert_eq!(t.count_possible("brgr"), 2);
        assert_eq!(t.count_possible("bbrgwb"), 0);
    }
}
