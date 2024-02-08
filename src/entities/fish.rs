use sqlx::{MySql, Pool};
use crate::entities::parameter::Parameter;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Fish {
    user_id: i64,
    tank_id: i64,
    fish_id: i64,
    name: String,
    species: String,
    qty: i64
}

// CREATE TABLE `fish` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `fishid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `species` VARCHAR(255) NOT NULL,
// `qty` INT,
// PRIMARY KEY (`fishid`)
// ) ENGINE=InnoDB;


pub fn create_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}
pub fn delete_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}
pub fn modify_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}
