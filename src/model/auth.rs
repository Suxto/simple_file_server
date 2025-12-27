pub struct Session {
    pub username: String,
    pub token: String,
    pub expires_at: i64,
    pub created_at: i64,
    pub last_used_at: i64,
    pub max_age: i64,
    pub max_idle_time: i64,
    pub max_requests: i64,
    pub requests: i64,
    pub ip: String,
    pub user_agent: String,
}