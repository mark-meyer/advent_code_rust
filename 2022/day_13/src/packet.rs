use std::cmp::Ordering;
use std::iter::zip;
use crate::Packet::*;
use std::str::FromStr;

#[derive(Debug, Eq, Ord, Clone)]
pub enum Packet{
    List(Vec<Packet>),
    Int(usize)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePacketError;

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars().peekable();
        let mut stack = vec![];

        while let Some(c) =  it.next() {
            if c  == '[' {
                stack.push(List(vec![]));
            }
            if c == ']' {
                let last = stack.pop().ok_or(ParsePacketError)?;
                match stack.last_mut() {
                    Some(List(l)) => l.push(last),
                    None => stack.push(last),
                    _ => return Err(ParsePacketError)
                }
            }
            if c.is_digit(10) {
                let mut n = c.to_string();
                while let Some(d) =  it.next_if(|d| d.is_digit(10)) {
                    n.push(d)
                }
                // naked INTs are not allowed - packets are always lists
                if let List(l) = stack.last_mut().ok_or(ParsePacketError)? {
                    l.push(Int(n.parse().unwrap()));
                }
            }
        }
        stack.pop().ok_or(ParsePacketError)

    }
}


impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Int(a),  Int(b))  => a == b,
            (List(a), List(b)) => a == b,
            _ => false
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(a),  Int(b))  => a.partial_cmp(b),
            (Int(a),  List(_)) => List(vec![Int(*a)]).partial_cmp(other),
            (List(_), Int(b))  => self.partial_cmp(&List(vec![Int(*b)])),
            (List(a), List(b)) => {
                zip(a, b)
                    .map(|(i, j)| i.partial_cmp(j))
                    .filter(|o| {
                        match o {
                            Some(Ordering::Equal)  => false,
                            _ => true
                        }
                    })
                    .next()
                    .unwrap_or(a.len().partial_cmp(&b.len()))
            }
        }
    }
}
