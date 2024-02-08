use crate::entities::fish::Fish;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tank {
    user_id: i64,
    name: String,
    size_gallons: String,
    height: i64,
    length: i64,
    width: i64,
    volume: i64,
    weight: i64,
    occupants: Vec<Fish>
}

// CREATE TABLE `tank` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `size_gallons` VARCHAR(255) NOT NULL,
// `height` INT,
// `length` INT,
// `width` INT,
// `volume` INT,
// `weight` INT,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;
