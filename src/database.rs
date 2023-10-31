use crate::models::{NewRatify, Ratify};
use crate::schema;
use anyhow::Error;
use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

lazy_static! {
    static ref PG_POOL: PgPool = PgPool::new();
}

struct PgPool(Pool<ConnectionManager<PgConnection>>);

impl PgPool {
    fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .map_err(|err| panic!("{}", err))
            .unwrap();
        Self(pool)
    }
    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        let conn = self.0.get()?;
        Ok(conn)
    }
}

pub fn insert_ratify(new_ratify: NewRatify) -> Result<(), Error> {
    use schema::ratify;
    let mut conn = PG_POOL.get_conn()?;

    diesel::insert_into(ratify::table)
        .values(new_ratify)
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_ratifications(height: i64) -> Result<Vec<Ratify>, Error> {
    use schema::ratify::dsl;
    let mut conn = PG_POOL.get_conn()?;

    let result = dsl::ratify
        .filter(dsl::height.eq(height))
        .select(Ratify::as_select())
        .load(&mut conn)?;

    Ok(result)
}
