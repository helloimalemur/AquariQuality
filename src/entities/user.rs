use sqlx::{MySql, Pool};
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

pub fn create_user(user: User, db_pool: Pool<MySql>) {}
pub fn delete_user(user: User, db_pool: Pool<MySql>) {}
pub fn modify_user(user: User, db_pool: Pool<MySql>) {}
