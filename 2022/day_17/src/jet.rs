#[derive(Debug, Default)]
pub enum Jet {
    #[default]
    Right,
    Left
}


impl From<char> for Jet {
    fn from(c: char) -> Jet {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Not a jet code")
        }
    }
}