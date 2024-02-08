use crate::entities::fish::Fish;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tank {
    name: String,
    size_gallons: String,
    height: i64,
    length: i64,
    width: i64,
    volume: i64,
    weight: i64,
    occupants: Vec<Fish>
}
