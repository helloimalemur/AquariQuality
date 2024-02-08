use crate::entities::tank::Tank;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct User {
    user_id: i64,
    name: String,
    email: String,
    tanks: Vec<Tank>
}


// CREATE TABLE `user` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;
