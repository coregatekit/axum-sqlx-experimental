use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool
}