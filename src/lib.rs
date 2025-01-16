// Private modules
mod error;

// Public facing modules
pub mod server;
pub mod db;
pub mod handlers;
pub mod services;
pub mod util;

// Re-exports
pub use error::Error;
pub use error::Result;
use sqlx::MySqlPool;

pub struct AppState {
    pub pool: MySqlPool,
}
