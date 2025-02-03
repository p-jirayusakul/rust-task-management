use mockall::automock;
use sonyflake::Sonyflake;

#[automock]
pub trait Snowflake {
    fn generate(&self) -> u64;
}

pub struct SnowflakeImpl {
    snowflake_id: Sonyflake,
}

impl SnowflakeImpl {
    pub fn new(snowflake_id: Sonyflake) -> Self {
        Self { snowflake_id }
    }
}

impl Snowflake for SnowflakeImpl {
    fn generate(&self) -> u64 {
        self.snowflake_id.next_id().expect("Failed to generate next Sonyflake ID")
    }
}
pub fn initialize_sonyflake() -> Result<Sonyflake, std::io::Error> {
    let sonyflake = Sonyflake::new().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize Sonyflake: {}", e),
        )
    })?;
    Ok(sonyflake)
}