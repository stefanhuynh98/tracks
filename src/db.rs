use dotenvy_macro::dotenv;
use sqlx::MySqlPool;

use crate::Result;

pub fn create_pool() -> Result<MySqlPool> {
    let db_url = dotenv!("DATABASE_URL");
    let pool = MySqlPool::connect_lazy(db_url)?;

    Ok(pool)
}
