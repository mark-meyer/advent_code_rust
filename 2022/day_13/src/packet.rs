use serde_json::Value;
use std::cmp::Ordering;
use std::iter::zip;
use serde_json::Value::{Number, Array};
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
        match serde_json::from_str(s) {
            Ok(v) =>  Ok(Packet::from_serde_value(&v)?),
            Err(_) => Err(ParsePacketError)
        }
    }
}

impl Packet {
    fn from_serde_value(v: &Value) -> Result<Self, ParsePacketError> {
        match v {
            Number(_) => Ok(Int(v.as_u64().unwrap() as usize)),
            Array(_) => {
                match v.as_array().unwrap().iter().map(|val| Packet::from_serde_value(val)).collect() {
                    Ok(c) => Ok(List(c)),
                    Err(_) => Err(ParsePacketError)
                }
            },
            _ => panic!("Not a packet!")
        }    
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
