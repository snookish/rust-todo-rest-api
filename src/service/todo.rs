use diesel::{
    MysqlConnection,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct Service {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Service {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Result<Self, Error> {
        Ok(Self { pool })
    }
}
