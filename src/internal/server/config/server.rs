use std::env;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerConfig {
    pub database_host: String,
    pub database_port: u16,
    pub database_name: String,
    pub database_user: String,
    pub database_password: String,
    pub jwt_secret: String,
    pub api_port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, std::io::Error> {
        Ok(Self {
            database_host: env::var("DB_HOST").expect("DB_HOST must be set in environment variables"),
            database_port: env::var("DB_PORT")
                .expect("DB_PORT must be set in environment variables")
                .parse::<u16>()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid DB_PORT: {}", e)))?,
            database_name: env::var("DB_DATABASE").expect("DB_DATABASE must be set in environment variables"),
            database_user: env::var("DB_USERNAME").expect("DB_USERNAME must be set in environment variables"),
            database_password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in environment variables"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment variables"),
            api_port: parse_port_from_env()?,
        })
    }
}

pub fn load_env(env_file: &str) {
    if dotenv::from_filename(env_file).is_err() {
        println!("Warning: {} not found. Using OS environment variables instead.", env_file);
    }
    for &key in &["APP_PORT", "DB_HOST", "DB_PORT", "DB_DATABASE", "DB_USERNAME", "DB_PASSWORD", "JWT_SECRET"] {
        if env::var(key).is_err() {
            println!("Warning: Environment variable {} is not set!", key);
        }
    }
}

pub fn parse_port_from_env() -> Result<u16, std::io::Error> {
    let port_string = env::var("APP_PORT").unwrap_or_else(|_| "8080".into());
    port_string.parse::<u16>().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid APP_PORT: {}", e))
    })
}