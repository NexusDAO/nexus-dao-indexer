use anyhow::Error;
use diesel::{
    r2d2::{ConnectionManager, PoolError},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

use crate::{models::Record, schema};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<PgConnection>> = create_pg_pool().unwrap();
}

pub fn get_records_by_height(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    start_block: i64,
    end_block: i64,
) -> Result<Vec<Record>, Error> {
    use schema::record::dsl::*;

    let records = record
        .filter(height.between(start_block, end_block))
        .select(Record::as_select())
        .load(conn)
        .expect("Error loading records");

    Ok(records)
}

fn create_pg_pool() -> Result<PgPool, PoolError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    PgPool::builder().build(manager)
}
