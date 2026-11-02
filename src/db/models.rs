use diesel::data_types;
use diesel::prelude::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::todos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: data_types::MysqlTime,
    pub updated_at: data_types::MysqlTime,
}
