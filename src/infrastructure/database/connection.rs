use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> anyhow::Result<PgPool> {
    let database_url =
        env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .test_on_check_out(true)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .map_err(|e| anyhow::anyhow!("Failed to create connection pool: {}", e))
}
