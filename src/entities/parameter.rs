#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Parameter {
    user_id: i64,
    ph: i64,
    kh: i64,
}


// CREATE TABLE `parameter` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `ph` INT,
// `kh` INT,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;
