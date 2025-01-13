use std::env;

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, std::io::Error> {
        Ok(Self {
            host: env::var("DB_HOST").expect("DB_HOST must be set in environment variables"),
            port: env::var("DB_PORT")
                .expect("DB_PORT must be set in environment variables")
                .parse::<u16>()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid DB_PORT: {}", e)))?,
            name: env::var("DB_DATABASE").expect("DB_DATABASE must be set in environment variables"),
            user: env::var("DB_USERNAME").expect("DB_USERNAME must be set in environment variables"),
            password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in environment variables"),
        })
    }
}

pub fn load_env(env_file: &str) {
    if dotenv::from_filename(env_file).is_err() {
        println!("Warning: {} not found. Using OS environment variables instead.", env_file);
    }
    for &key in &["APP_PORT", "DB_HOST", "DB_PORT", "DB_DATABASE", "DB_USERNAME", "DB_PASSWORD"] {
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