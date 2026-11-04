use diesel::{
    Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    data_types::MysqlTime,
    r2d2::{self, ConnectionManager, Pool},
};

use crate::db::{
    models::{self, CreateTodoCmd, Todo},
    pool::{DbError, DbPool},
    schema,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database connection pool error: {0}")]
    Pool(#[from] DbError),

    #[error("Database error: {0}")]
    Diesel(#[from] diesel::result::Error),
}

pub struct Service {
    pool: DbPool,
}

impl Service {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, cmd: CreateTodoCmd) -> Result<Todo, Error> {
        let mut conn = self.pool.get().map_err(DbError::from)?;
        let todo = conn.transaction(|tx| {
            diesel::insert_into(schema::todos::table)
                .values(&cmd)
                .execute(tx)?;

            schema::todos::table
                .order(schema::todos::id.desc())
                .select(models::Todo::as_select())
                .first(tx)
        })?;

        Ok(todo)
    }
}
