use sqlx::{MySql, Pool};
use crate::entities::tank::Tank;

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

pub fn create_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}
pub fn delete_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}
pub fn modify_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}
