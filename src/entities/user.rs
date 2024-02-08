use crate::entities::tank::Tank;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct User {
    name: String,
    email: String,
    tanks: Vec<Tank>
}
